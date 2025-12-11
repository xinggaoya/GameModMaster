// 导入的引用
use crate::api::error::{AppError, AppResult};
use crate::api::trainer::DownloadProgress;
use crate::services::settings;
use futures_util::StreamExt;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tauri::Emitter;

// 下载管理器状态
lazy_static! {
    static ref ACTIVE_DOWNLOADS: Arc<Mutex<HashMap<String, DownloadProgress>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

// 获取所有活动下载的进度
#[tauri::command]
pub fn get_active_downloads() -> Vec<DownloadProgress> {
    let downloads = ACTIVE_DOWNLOADS.lock().unwrap();
    downloads.values().cloned().collect()
}

// 通用文件下载函数
pub async fn download_file_with_progress<R: tauri::Runtime>(
    app_handle: tauri::AppHandle<R>,
    url: &str,
    file_id: &str,
    save_path: &PathBuf,
    event_name: &str,
) -> AppResult<()> {
    // 确保父目录存在
    if let Some(parent) = save_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // 准备进度跟踪
    let file_id = file_id.to_string();
    let mut last_notify_time = Instant::now();

    // 下载文件
    let client = reqwest::Client::new();
    let res = client.get(url).send().await?;

    if !res.status().is_success() {
        return Err(AppError::DownloadError(format!(
            "下载失败，HTTP状态码: {}",
            res.status()
        )));
    }

    // 获取文件总大小
    let total_size = res.content_length();

    // 发送初始进度
    let progress = DownloadProgress {
        trainer_id: file_id.clone(), // 在这里用作文件ID
        progress: 0.0,
        downloaded_bytes: 0,
        total_bytes: total_size,
        status: "downloading".to_string(),
        speed: None,
    };

    // 保存到活动下载列表
    {
        let mut downloads = ACTIVE_DOWNLOADS.lock().unwrap();
        downloads.insert(file_id.clone(), progress.clone());
    }

    // 发送进度事件
    let _ = app_handle.emit(event_name, progress);

    // 准备写入文件
    let mut file = fs::File::create(save_path)?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();
    let mut last_downloaded = 0u64;
    let mut last_speed_check = Instant::now();

    // 处理流式下载
    while let Some(item) = stream.next().await {
        let chunk = item?;
        file.write_all(&chunk)?;

        downloaded += chunk.len() as u64;

        // 计算进度百分比
        let percent = if let Some(total) = total_size {
            (downloaded as f64 / total as f64) * 100.0
        } else {
            0.0 // 无法确定总大小
        };

        // 计算下载速度（每秒更新一次）
        let mut current_speed = None;
        if last_speed_check.elapsed() >= Duration::from_secs(1) {
            let elapsed = last_speed_check.elapsed().as_secs_f64();
            if elapsed > 0.0 {
                let bytes_since_last = downloaded - last_downloaded;
                let speed_kbps = (bytes_since_last as f64 / elapsed) / 1024.0; // KB/s
                current_speed = Some(speed_kbps);

                last_speed_check = Instant::now();
                last_downloaded = downloaded;
            }
        }

        // 限制进度通知频率，避免过多的事件
        if last_notify_time.elapsed() >= Duration::from_millis(200) {
            let progress = DownloadProgress {
                trainer_id: file_id.clone(),
                progress: percent,
                downloaded_bytes: downloaded,
                total_bytes: total_size,
                status: "downloading".to_string(),
                speed: current_speed,
            };

            // 更新活动下载列表
            {
                let mut downloads = ACTIVE_DOWNLOADS.lock().unwrap();
                downloads.insert(file_id.clone(), progress.clone());
            }

            // 发送进度事件
            let _ = app_handle.emit(event_name, progress);
            last_notify_time = Instant::now();
        }
    }

    file.flush()?;

    // 发送完成进度
    let final_progress = DownloadProgress {
        trainer_id: file_id.clone(),
        progress: 100.0,
        downloaded_bytes: downloaded,
        total_bytes: total_size,
        status: "completed".to_string(),
        speed: None,
    };
    let _ = app_handle.emit(event_name, final_progress);

    // 从活动下载列表移除
    {
        let mut downloads = ACTIVE_DOWNLOADS.lock().unwrap();
        downloads.remove(&file_id);
    }

    Ok(())
}

// 取消下载
#[tauri::command]
pub fn cancel_download(file_id: String) -> AppResult<bool> {
    let mut downloads = ACTIVE_DOWNLOADS.lock().unwrap();
    if downloads.contains_key(&file_id) {
        downloads.remove(&file_id);
        Ok(true)
    } else {
        Ok(false)
    }
}

// 清理所有下载
#[tauri::command]
pub fn clear_all_downloads() -> AppResult<usize> {
    let mut downloads = ACTIVE_DOWNLOADS.lock().unwrap();
    let count = downloads.len();
    downloads.clear();
    Ok(count)
}

// 获取下载目录
#[tauri::command]
pub fn get_download_directory() -> AppResult<String> {
    let download_dir = settings::get_download_path()?;
    Ok(download_dir.to_string_lossy().to_string())
}

// 打开下载目录
#[tauri::command]
pub fn open_download_directory() -> AppResult<()> {
    let download_dir = settings::get_download_path()?;

    #[cfg(target_os = "windows")]
    {
        use std::ffi::OsStr;
        use std::os::windows::ffi::OsStrExt;
        use std::ptr;
        use windows_sys::Win32::UI::Shell::ShellExecuteW;
        use windows_sys::Win32::UI::WindowsAndMessaging::SW_SHOW;

        let dir_path = download_dir.to_string_lossy().into_owned();
        let mut wide_path: Vec<u16> = OsStr::new(&dir_path)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        let result = unsafe {
            ShellExecuteW(
                0,
                windows_str("explore"),
                wide_path.as_mut_ptr(),
                ptr::null_mut(),
                ptr::null_mut(),
                SW_SHOW,
            )
        };

        if result <= 32 {
            return Err(AppError::ExecutionError(format!(
                "打开下载目录失败，错误码: {}",
                result
            )));
        }
    }

    #[cfg(target_os = "macos")]
    {
        use std::process::Command;

        let _ = Command::new("open").arg(&download_dir).spawn()?;
    }

    #[cfg(target_os = "linux")]
    {
        use std::process::Command;

        let _ = Command::new("xdg-open").arg(&download_dir).spawn()?;
    }

    Ok(())
}

// 辅助函数：把字符串转为Windows宽字符
#[cfg(target_os = "windows")]
fn windows_str(s: &str) -> *mut u16 {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

    let mut wide: Vec<u16> = OsStr::new(s)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    wide.as_mut_ptr()
}
