use directories::ProjectDirs;
use std::path::PathBuf;
use crate::api::error::{AppError, AppResult};

pub fn get_app_dir() -> AppResult<PathBuf> {
    ProjectDirs::from("com", "trainer", "manager")
        .map(|dirs| dirs.data_dir().to_path_buf())
        .ok_or_else(|| AppError::ConfigError("Failed to get app directory".to_string()))
}

pub fn get_downloads_dir() -> AppResult<PathBuf> {
    Ok(get_app_dir()?.join("downloads"))
}

pub fn sanitize_filename(filename: &str) -> String {
    // Windows 文件名中的非法字符
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

    // 移除前后的空格和点
    safe_name = safe_name
        .trim()
        .trim_matches(|c: char| c == '.' || !c.is_ascii_graphic())
        .to_string();

    // Windows 保留文件名检查
    let reserved_names = [
        "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5",
        "COM6", "COM7", "COM8", "COM9", "LPT1", "LPT2", "LPT3", "LPT4",
        "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
    ];

    if reserved_names.contains(&safe_name.to_uppercase().as_str()) {
        safe_name = format!("_{}", safe_name);
    }

    // 空文件名处理
    if safe_name.is_empty() {
        safe_name = "unnamed".to_string();
    }

    // 文件名长度限制
    if safe_name.len() > 240 {
        safe_name = safe_name[..240].to_string();
    }

    safe_name
        .trim_end_matches(|c: char| c == '.' || c.is_whitespace())
        .to_string()
} 