use crate::api::error::{AppError, AppResult};
use crate::api::trainer::PaginatedResponse;
use crate::models::trainer::{Trainer, TrainerInstallInfo};
use crate::services::download_manager;
use crate::services::scraper;
use crate::utils::path::{get_downloads_dir, sanitize_filename};
use crate::utils::zip::extract_zip;
use chrono::Local;
use std::fs;
use std::io::Read;
#[cfg(target_os = "windows")]
use std::os::windows::ffi::OsStrExt;
use std::path::PathBuf;
use tauri::Emitter;
use tauri::Manager;
#[cfg(target_os = "windows")]
use windows_sys::Win32::UI::Shell::ShellExecuteW;
#[cfg(target_os = "windows")]
use windows_sys::Win32::UI::WindowsAndMessaging::SW_SHOW;

pub async fn fetch_trainers(page: u32) -> AppResult<PaginatedResponse<Trainer>> {
    let url = format!("https://flingtrainer.com/page/{}/", page);
    let response = reqwest::get(&url).await?;
    let html = response.text().await?;
    let trainers = scraper::parse_trainer_list(&html)?;

    // 这里需要从网页中解析总数，暂时使用固定值
    let total = 120; // 假设总共有120个训练器

    Ok(PaginatedResponse { trainers, total })
}

pub async fn search_trainers(query: String, page: u32) -> AppResult<PaginatedResponse<Trainer>> {
    let url = format!("https://flingtrainer.com/page/{}/?s={}", page, query);
    let response = reqwest::get(&url).await?;
    let html = response.text().await?;
    let trainers = scraper::parse_trainer_list(&html)?;

    // 这里需要从搜索结果页面解析总数，暂时使用固定值
    let total = trainers.len() as u32;

    Ok(PaginatedResponse { trainers, total })
}

pub async fn get_trainer_detail(id: String) -> AppResult<Trainer> {
    let url = format!("https://flingtrainer.com/trainer/{}/", id);
    let response = reqwest::get(&url).await?;
    let html = response.text().await?;
    scraper::parse_trainer_detail(&html)
}

pub async fn download_trainer<R: tauri::Runtime>(
    app_handle: tauri::AppHandle<R>,
    trainer: Trainer,
) -> AppResult<PathBuf> {
    println!(
        "开始下载修改器: {} ({})",
        trainer.name, trainer.download_url
    );

    let download_dir = get_downloads_dir()?;
    fs::create_dir_all(&download_dir)?;

    // 生成标准化的修改器目录名
    let safe_name = sanitize_filename(&trainer.name);
    let trainer_dir_name = format!("{}_{}", safe_name, trainer.id);
    let trainer_dir = download_dir.join(&trainer_dir_name);

    // 如果目录已存在，先删除
    if trainer_dir.exists() {
        fs::remove_dir_all(&trainer_dir)?;
    }

    // 创建修改器目录
    fs::create_dir_all(&trainer_dir)?;

    // 临时zip文件
    let temp_zip = download_dir.join(format!("temp_{}.zip", trainer.id));
    if temp_zip.exists() {
        fs::remove_file(&temp_zip)?;
    }

    // 使用下载管理器下载文件
    download_manager::download_file_with_progress(
        app_handle.clone(),
        &trainer.download_url,
        &trainer.id,
        &temp_zip,
        "download-progress",
    )
    .await?;

    // 验证临时文件
    if !temp_zip.exists() || fs::metadata(&temp_zip)?.len() == 0 {
        return Err(AppError::DownloadError(
            "Failed to create temporary file".to_string(),
        ));
    }

    // 检查文件类型
    let is_zip_file = is_zip_file(&temp_zip);
    let is_exe_file = is_exe_file(&temp_zip);

    println!("文件类型检测: ZIP={}, EXE={}", is_zip_file, is_exe_file);

    // 发送处理进度
    let _ = app_handle.emit(
        "download-progress",
        serde_json::json!({
            "trainer_id": trainer.id,
            "status": "processing",
            "progress": 100.0,
            "downloaded_bytes": fs::metadata(&temp_zip)?.len(),
            "total_bytes": fs::metadata(&temp_zip)?.len(),
            "speed": null
        }),
    );

    // 处理下载的文件
    if is_zip_file {
        // 如果是ZIP文件，解压
        println!("检测到ZIP文件，开始解压...");
        if let Err(e) = extract_zip(&temp_zip, &trainer_dir) {
            println!("解压失败: {}", e);
            fs::remove_dir_all(&trainer_dir)?;
            fs::remove_file(&temp_zip)?;
            return Err(e);
        }
        // 解压后删除临时文件
        fs::remove_file(&temp_zip)?;
    } else if is_exe_file {
        // 如果是EXE文件，直接移动到目标目录
        println!("检测到EXE文件，直接使用...");
        let exe_filename = format!("{}.exe", trainer.id);
        let target_exe_path = trainer_dir.join(&exe_filename);
        fs::rename(&temp_zip, &target_exe_path)?;
    } else {
        // 未知文件类型，尝试作为ZIP处理
        println!("未知文件类型，尝试作为ZIP处理...");
        if let Err(e) = extract_zip(&temp_zip, &trainer_dir) {
            println!("解压失败，尝试直接复制文件: {}", e);
            // 解压失败，直接复制文件到目标目录
            let target_file_path = trainer_dir.join(format!("unknown_file_{}.bin", trainer.id));
            fs::copy(&temp_zip, &target_file_path)?;
            fs::remove_file(&temp_zip)?;
        } else {
            // 解压成功，删除临时文件
            fs::remove_file(&temp_zip)?;
        }
    }

    // 保存安装信息
    let install_info = TrainerInstallInfo {
        trainer: trainer.clone(),
        install_path: trainer_dir.to_string_lossy().to_string(),
        install_time: Local::now().to_rfc3339(),
        last_launch_time: None,
    };

    let info_json = serde_json::to_string_pretty(&install_info)?;
    fs::write(trainer_dir.join("trainer.json"), info_json)?;

    // 发送完成进度
    let _ = app_handle.emit(
        "download-progress",
        serde_json::json!({
            "trainer_id": trainer.id,
            "status": "completed",
            "progress": 100.0,
            "downloaded_bytes": fs::metadata(&temp_zip).map(|m| m.len()).unwrap_or(0),
            "total_bytes": fs::metadata(&temp_zip).map(|m| m.len()).unwrap_or(0),
            "speed": null
        }),
    );

    Ok(trainer_dir)
}

// 检测文件是否为ZIP格式
fn is_zip_file(file_path: &PathBuf) -> bool {
    // 检查文件头部是否为ZIP格式的特征码 (PK\x03\x04)
    let mut file = match std::fs::File::open(file_path) {
        Ok(f) => f,
        Err(_) => return false,
    };

    let mut header_buffer = [0u8; 4];
    if let Ok(bytes_read) = file.read(&mut header_buffer) {
        if bytes_read >= 4 {
            // ZIP文件的头部应该是PK\x03\x04
            return header_buffer[0] == 0x50
                && header_buffer[1] == 0x4B
                && header_buffer[2] == 0x03
                && header_buffer[3] == 0x04;
        }
    }

    false
}

// 检测文件是否为EXE格式
fn is_exe_file(file_path: &PathBuf) -> bool {
    // 检查文件头部是否为EXE格式的特征码 (MZ)
    let mut file = match std::fs::File::open(file_path) {
        Ok(f) => f,
        Err(_) => return false,
    };

    let mut header_buffer = [0u8; 2];
    if let Ok(bytes_read) = file.read(&mut header_buffer) {
        if bytes_read >= 2 {
            // EXE文件的头部应该是MZ (0x4D 0x5A)
            return header_buffer[0] == 0x4D && header_buffer[1] == 0x5A;
        }
    }

    false
}

pub async fn launch_trainer(trainer_id: String) -> AppResult<()> {
    let download_dir = get_downloads_dir()?;
    let mut trainer_path = None;
    let mut executable_path = None;

    if let Ok(entries) = fs::read_dir(&download_dir) {
        for entry in entries.flatten() {
            if let Ok(path) = entry.path().canonicalize() {
                if path.is_dir() {
                    let info_path = path.join("trainer.json");
                    if info_path.exists() {
                        if let Ok(content) = fs::read_to_string(&info_path) {
                            if let Ok(install_info) =
                                serde_json::from_str::<TrainerInstallInfo>(&content)
                            {
                                if install_info.trainer.id == trainer_id {
                                    trainer_path = Some(path);
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let trainer_dir = match trainer_path {
        Some(path) => path,
        None => return Err(AppError::NotFoundError("修改器未找到".to_string())),
    };

    // 查找可执行文件 (EXE)
    if let Ok(entries) = fs::read_dir(&trainer_dir) {
        for entry in entries.flatten() {
            if let Ok(path) = entry.path().canonicalize() {
                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        if ext.to_ascii_lowercase() == "exe" {
                            executable_path = Some(path);
                            break;
                        }
                    }
                }
            }
        }
    }

    // 如果没有找到EXE文件，尝试使用trainer_id.exe
    if executable_path.is_none() {
        let default_exe = trainer_dir.join(format!("{}.exe", trainer_id));
        if default_exe.exists() {
            executable_path = Some(default_exe);
        }
    }

    let exe_path = match executable_path {
        Some(path) => path,
        None => return Err(AppError::NotFoundError("可执行文件未找到".to_string())),
    };

    println!("启动修改器: {}", exe_path.display());

    #[cfg(target_os = "windows")]
    {
        use std::ffi::OsStr;
        use std::ptr;

        let exe_path_str = exe_path.to_string_lossy().into_owned();
        let mut wide_path: Vec<u16> = OsStr::new(&exe_path_str)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        let result = unsafe {
            ShellExecuteW(
                0,
                ptr::null_mut(),
                wide_path.as_mut_ptr(),
                ptr::null_mut(),
                ptr::null_mut(),
                SW_SHOW,
            )
        };

        if result <= 32 {
            return Err(AppError::ExecutionError(format!(
                "启动修改器失败，错误码: {}",
                result
            )));
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        // 非Windows系统的启动逻辑
        use std::process::Command;

        let status = Command::new(&exe_path).spawn();

        if let Err(e) = status {
            return Err(AppError::ExecutionError(format!("启动修改器失败: {}", e)));
        }
    }

    // 更新最后启动时间
    if let Ok(content) = fs::read_to_string(trainer_dir.join("trainer.json")) {
        if let Ok(mut install_info) = serde_json::from_str::<TrainerInstallInfo>(&content) {
            install_info.last_launch_time = Some(Local::now().to_rfc3339());
            let updated_json = serde_json::to_string_pretty(&install_info)?;
            fs::write(trainer_dir.join("trainer.json"), updated_json)?;
        }
    }

    Ok(())
}

pub fn delete_trainer(trainer_id: String) -> AppResult<()> {
    let download_dir = get_downloads_dir()?;

    if let Ok(entries) = fs::read_dir(download_dir) {
        for entry in entries.flatten() {
            if let Ok(path) = entry.path().canonicalize() {
                if path.is_dir() {
                    let info_path = path.join("trainer.json");
                    if info_path.exists() {
                        if let Ok(content) = fs::read_to_string(&info_path) {
                            if let Ok(install_info) =
                                serde_json::from_str::<TrainerInstallInfo>(&content)
                            {
                                if install_info.trainer.id == trainer_id {
                                    fs::remove_dir_all(path)?;
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
