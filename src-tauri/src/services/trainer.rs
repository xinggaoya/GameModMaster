use std::fs;
use std::path::PathBuf;
use std::io::{Write, Read};
use chrono::Local;
#[cfg(target_os = "windows")]
use windows_sys::Win32::UI::Shell::ShellExecuteW;
#[cfg(target_os = "windows")]
use windows_sys::Win32::UI::WindowsAndMessaging::SW_SHOW;
#[cfg(target_os = "windows")]
use std::os::windows::ffi::OsStrExt;
use crate::api::error::{AppError, AppResult};
use crate::models::trainer::{Trainer, TrainerInstallInfo};
use crate::utils::path::{get_downloads_dir, sanitize_filename};
use crate::utils::zip::extract_zip;
use crate::services::scraper;
use crate::api::trainer::PaginatedResponse;

pub async fn fetch_trainers(page: u32) -> AppResult<PaginatedResponse<Trainer>> {
    let url = format!("https://flingtrainer.com/page/{}/", page);
    let response = reqwest::get(&url).await?;
    let html = response.text().await?;
    let trainers = scraper::parse_trainer_list(&html)?;

    // 这里需要从网页中解析总数，暂时使用固定值
    let total = 120; // 假设总共有120个训练器

    Ok(PaginatedResponse {
        trainers,
        total,
    })
}

pub async fn search_trainers(query: String, page: u32) -> AppResult<PaginatedResponse<Trainer>> {
    let url = format!("https://flingtrainer.com/page/{}/?s={}", page, query);
    let response = reqwest::get(&url).await?;
    let html = response.text().await?;
    let trainers = scraper::parse_trainer_list(&html)?;

    // 这里需要从搜索结果页面解析总数，暂时使用固定值
    let total = trainers.len() as u32;

    Ok(PaginatedResponse {
        trainers,
        total,
    })
}

pub async fn get_trainer_detail(id: String) -> AppResult<Trainer> {
    let url = format!("https://flingtrainer.com/trainer/{}/", id);
    let response = reqwest::get(&url).await?;
    let html = response.text().await?;
    scraper::parse_trainer_detail(&html)
}

pub async fn download_trainer(trainer: Trainer) -> AppResult<PathBuf> {
    println!("开始下载修改器: {} ({})", trainer.name, trainer.download_url);

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

    // 下载文件
    let response = reqwest::get(&trainer.download_url).await?;
    if !response.status().is_success() {
        return Err(AppError::DownloadError(format!(
            "Download failed with status: {}",
            response.status()
        )));
    }

    // 临时zip文件
    let temp_zip = download_dir.join("temp.zip");
    if temp_zip.exists() {
        fs::remove_file(&temp_zip)?;
    }

    // 写入临时文件
    {
        let mut file = fs::File::create(&temp_zip)?;
        let content = response.bytes().await?;
        std::io::copy(&mut content.as_ref(), &mut file)?;
        file.flush()?;
    }

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

    Ok(trainer_dir)
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
                            if let Ok(install_info) = serde_json::from_str::<TrainerInstallInfo>(&content) {
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
            return header_buffer[0] == 0x50 && header_buffer[1] == 0x4B &&
                   header_buffer[2] == 0x03 && header_buffer[3] == 0x04;
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

    if let Ok(entries) = fs::read_dir(download_dir) {
        for entry in entries.flatten() {
            if let Ok(path) = entry.path().canonicalize() {
                if path.is_dir() {
                    let info_path = path.join("trainer.json");
                    if info_path.exists() {
                        if let Ok(content) = fs::read_to_string(&info_path) {
                            if let Ok(mut install_info) = serde_json::from_str::<TrainerInstallInfo>(&content) {
                                if install_info.trainer.id == trainer_id {
                                    // 搜索可执行文件
                                    if let Ok(files) = fs::read_dir(&path) {
                                        for file in files.flatten() {
                                            let file_path = file.path();
                                            if let Some(extension) = file_path.extension() {
                                                if extension == "exe" {
                                                    // 更新启动时间
                                                    install_info.last_launch_time = Some(Local::now().to_rfc3339());
                                                    let info_json = serde_json::to_string_pretty(&install_info)?;
                                                    fs::write(&info_path, info_json)?;

                                                    // 使用Shell执行并请求管理员权限
                                                    #[cfg(target_os = "windows")]
                                                    {
                                                        // 转换路径为宽字符
                                                        let wide_path: Vec<u16> = file_path.as_os_str()
                                                            .encode_wide()
                                                            .chain(Some(0))
                                                            .collect();
                                                        let wide_operation: Vec<u16> = "runas".encode_utf16().chain(Some(0)).collect();
                                                        let wide_dir: Vec<u16> = path.as_os_str()
                                                            .encode_wide()
                                                            .chain(Some(0))
                                                            .collect();

                                                        let result = unsafe {
                                                            ShellExecuteW(
                                                                0,
                                                                wide_operation.as_ptr(),
                                                                wide_path.as_ptr(),
                                                                std::ptr::null(),
                                                                wide_dir.as_ptr(),
                                                                SW_SHOW,
                                                            )
                                                        };

                                                        // ShellExecuteW返回值大于32表示成功
                                                        if result as isize <= 32 {
                                                            return Err(AppError::IoError(std::io::Error::new(
                                                                std::io::ErrorKind::PermissionDenied,
                                                                format!("启动修改器失败，需要管理员权限: {}", result),
                                                            )));
                                                        }

                                                        return Ok(());
                                                    }

                                                    // 非Windows平台的默认行为
                                                    #[cfg(not(target_os = "windows"))]
                                                    {
                                                        match Command::new(&file_path).spawn() {
                                                            Ok(_) => return Ok(()),
                                                            Err(e) => return Err(AppError::IoError(e)),
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        return Err(AppError::IoError(std::io::Error::new(
                                            std::io::ErrorKind::NotFound,
                                            "修改器目录中没有找到可执行文件",
                                        )));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Err(AppError::IoError(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "未找到修改器",
    )))
}