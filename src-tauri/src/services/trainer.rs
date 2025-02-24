use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::io::Write;
use chrono::Local;
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

    // 解压文件
    if let Err(e) = extract_zip(&temp_zip, &trainer_dir) {
        fs::remove_dir_all(&trainer_dir)?;
        fs::remove_file(&temp_zip)?;
        return Err(e);
    }

    // 清理临时文件
    fs::remove_file(&temp_zip)?;

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

                                                    // 启动程序
                                                    match Command::new(&file_path).spawn() {
                                                        Ok(_) => return Ok(()),
                                                        Err(e) => return Err(AppError::IoError(e)),
                                                    }
                                                }
                                            }
                                        }
                                        return Err(AppError::IoError(std::io::Error::new(
                                            std::io::ErrorKind::NotFound,
                                            "No executable file found in trainer directory",
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
        "Trainer not found",
    )))
} 