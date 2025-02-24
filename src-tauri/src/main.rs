// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod scraper;

use anyhow::Result;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use thiserror::Error;
use zip::ZipArchive;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Trainer {
    id: String,
    name: String,
    version: String,
    game_version: String,
    download_url: String,
    description: String,
    thumbnail: String,
    download_count: i32,
    last_update: String,
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Failed to parse HTML: {0}")]
    ParseError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Download error: {0}")]
    DownloadError(String),
}

impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

// 获取应用数据目录
fn get_app_dir() -> Option<PathBuf> {
    ProjectDirs::from("com", "trainer", "manager").map(|dirs| dirs.data_dir().to_path_buf())
}

// 获取修改器列表
#[tauri::command]
async fn fetch_trainers(page: u32) -> Result<Vec<Trainer>, AppError> {
    let url = format!("https://flingtrainer.com/page/{}/", page);
    let response = reqwest::get(&url).await?;
    let html = response.text().await?;
    scraper::parse_trainer_list(&html)
}

// 搜索修改器
#[tauri::command]
async fn search_trainers(query: String) -> Result<Vec<Trainer>, AppError> {
    let url = format!("https://flingtrainer.com/?s={}", query);
    let response = reqwest::get(&url).await?;
    let html = response.text().await?;
    scraper::parse_trainer_list(&html)
}

// 获取修改器详情
#[tauri::command]
async fn get_trainer_detail(id: String) -> Result<Trainer, AppError> {
    let url = format!("https://flingtrainer.com/trainer/{}/", id);
    let response = reqwest::get(&url).await?;
    let html = response.text().await?;
    scraper::parse_trainer_detail(&html)
}

// 辅助函数：清理文件名中的非法字符
fn sanitize_filename(filename: &str) -> String {
    // Windows 文件名中的非法字符和不建议使用的字符
    let invalid_chars = [
        '<', '>', ':', '"', '/', '\\', '|', '?', '*', '\0', '\n', '\r', '\t',
    ];
    let mut safe_name = filename
        .chars()
        .map(|c| {
            if invalid_chars.contains(&c) || !c.is_ascii() {
                '_'
            } else {
                c
            }
        })
        .collect::<String>();

    // 移除前后的空格、点和其他不可见字符
    safe_name = safe_name
        .trim()
        .trim_matches(|c: char| c == '.' || !c.is_ascii_graphic())
        .to_string();

    // Windows 保留文件名
    let reserved_names = [
        "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8",
        "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
    ];

    // 检查是否是保留名称
    if reserved_names.contains(&safe_name.to_uppercase().as_str()) {
        safe_name = format!("_{}", safe_name);
    }

    // 如果文件名为空，使用默认名称
    if safe_name.is_empty() {
        safe_name = "unnamed".to_string();
    }

    // 确保文件名不超过240个字符（为了安全留一些余地）
    if safe_name.len() > 240 {
        safe_name = safe_name[..240].to_string();
    }

    // 确保结尾没有空格和点
    safe_name
        .trim_end_matches(|c: char| c == '.' || c.is_whitespace())
        .to_string()
}

// 下载修改器
#[tauri::command]
async fn download_trainer(trainer: Trainer) -> Result<PathBuf, AppError> {
    println!(
        "开始下载修改器: {} ({})",
        trainer.name, trainer.download_url
    );

    let app_dir = get_app_dir().ok_or_else(|| {
        AppError::IoError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Failed to get app directory",
        ))
    })?;

    // 创建下载目录
    let download_dir = app_dir.join("downloads");
    std::fs::create_dir_all(&download_dir)?;

    // 生成标准化的修改器目录名
    let safe_name = sanitize_filename(&trainer.name);
    let safe_version = sanitize_filename(&trainer.version);
    let safe_game_version = sanitize_filename(&trainer.game_version);

    // 使用更简单的目录名格式
    let trainer_dir_name = format!("{}_{}", safe_name, trainer.id);
    let trainer_dir = download_dir.join(&trainer_dir_name);

    // 如果目录已存在，先删除
    if trainer_dir.exists() {
        std::fs::remove_dir_all(&trainer_dir)?;
    }

    // 创建修改器目录
    std::fs::create_dir_all(&trainer_dir)?;

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
    // 确保临时文件不存在
    if temp_zip.exists() {
        std::fs::remove_file(&temp_zip)?;
    }

    // 创建并写入临时文件
    {
        let mut file = std::fs::File::create(&temp_zip)?;
        let content = response.bytes().await?;
        std::io::copy(&mut content.as_ref(), &mut file)?;
        // 确保文件被正确写入和关闭
        file.flush()?;
    }

    // 验证临时文件是否存在且大小大于0
    if !temp_zip.exists() || std::fs::metadata(&temp_zip)?.len() == 0 {
        return Err(AppError::DownloadError(
            "Failed to create temporary file".to_string(),
        ));
    }

    // 解压文件
    match extract_zip(&temp_zip, &trainer_dir) {
        Ok(_) => {
            // 删除临时zip文件
            let _ = std::fs::remove_file(&temp_zip);

            // 保存修改器信息
            let trainer_info = serde_json::to_string_pretty(&trainer).map_err(|e| {
                AppError::IoError(std::io::Error::new(std::io::ErrorKind::Other, e))
            })?;
            let info_path = trainer_dir.join("trainer.json");
            std::fs::write(&info_path, trainer_info)?;

            Ok(trainer_dir)
        }
        Err(e) => {
            // 清理失败的目录和临时文件
            let _ = std::fs::remove_dir_all(&trainer_dir);
            let _ = std::fs::remove_file(&temp_zip);
            Err(AppError::IoError(e))
        }
    }
}

// 解压zip文件
fn extract_zip(zip_path: &PathBuf, extract_dir: &PathBuf) -> io::Result<()> {
    let file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => extract_dir.join(path),
            None => continue,
        };

        if (*file.name()).ends_with('/') {
            std::fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(p)?;
                }
            }
            let mut outfile = File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }

        // 获取并设置文件权限
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                std::fs::set_permissions(&outpath, std::fs::Permissions::from_mode(mode))?;
            }
        }
    }

    Ok(())
}

// 删除修改器
#[tauri::command]
fn delete_trainer(trainer_id: String) -> Result<(), AppError> {
    let app_dir = get_app_dir().ok_or_else(|| {
        AppError::IoError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Failed to get app directory",
        ))
    })?;

    let download_dir = app_dir.join("downloads");
    if let Ok(entries) = std::fs::read_dir(download_dir) {
        for entry in entries.flatten() {
            if let Ok(path) = entry.path().canonicalize() {
                if path.is_dir() {
                    // 检查trainer.json文件
                    let info_path = path.join("trainer.json");
                    if info_path.exists() {
                        if let Ok(content) = std::fs::read_to_string(&info_path) {
                            if let Ok(trainer_info) = serde_json::from_str::<Trainer>(&content) {
                                if trainer_info.id == trainer_id {
                                    std::fs::remove_dir_all(path)?;
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

// 启动修改器
#[tauri::command]
async fn launch_trainer(trainer_id: String) -> Result<(), AppError> {
    let app_dir = get_app_dir().ok_or_else(|| {
        AppError::IoError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Failed to get app directory",
        ))
    })?;

    let download_dir = app_dir.join("downloads");
    if let Ok(entries) = std::fs::read_dir(download_dir) {
        for entry in entries.flatten() {
            if let Ok(path) = entry.path().canonicalize() {
                if path.is_dir() {
                    // 检查trainer.json文件
                    let info_path = path.join("trainer.json");
                    if info_path.exists() {
                        if let Ok(content) = std::fs::read_to_string(&info_path) {
                            if let Ok(trainer_info) = serde_json::from_str::<Trainer>(&content) {
                                if trainer_info.id == trainer_id {
                                    println!("找到修改器目录: {:?}", path);

                                    // 搜索可执行文件
                                    if let Ok(files) = std::fs::read_dir(&path) {
                                        for file in files.flatten() {
                                            let file_path = file.path();
                                            if let Some(extension) = file_path.extension() {
                                                // 检查是否是可执行文件
                                                if extension == "exe" {
                                                    println!("找到可执行文件: {:?}", file_path);

                                                    // 使用std::process::Command启动程序
                                                    use std::process::Command;
                                                    match Command::new(&file_path).spawn() {
                                                        Ok(_) => {
                                                            println!("成功启动修改器");
                                                            return Ok(());
                                                        }
                                                        Err(e) => {
                                                            println!("启动失败: {}", e);
                                                            return Err(AppError::IoError(e));
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        // 如果没有找到exe文件，返回错误
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

    // 如果没有找到对应的修改器，返回错误
    Err(AppError::IoError(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "Trainer not found",
    )))
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            fetch_trainers,
            search_trainers,
            get_trainer_detail,
            download_trainer,
            delete_trainer,
            launch_trainer
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
