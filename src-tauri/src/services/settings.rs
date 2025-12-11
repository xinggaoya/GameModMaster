use crate::api::error::{AppError, AppResult};
use crate::utils::path::get_app_dir;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// 应用设置结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    /// 下载路径
    pub download_path: String,
    /// 主题设置 ("light" | "dark" | "system")
    pub theme: String,
    /// 是否自动解压
    pub auto_extract: bool,
    /// 下载完成后是否自动打开文件夹
    pub auto_open_folder: bool,
    /// 语言设置
    pub language: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        let default_download_path = get_default_download_path()
            .unwrap_or_else(|_| PathBuf::from("downloads"))
            .to_string_lossy()
            .to_string();

        Self {
            download_path: default_download_path,
            theme: "system".to_string(),
            auto_extract: true,
            auto_open_folder: false,
            language: "zh-CN".to_string(),
        }
    }
}

/// 获取默认下载路径
fn get_default_download_path() -> AppResult<PathBuf> {
    Ok(get_app_dir()?.join("downloads"))
}

/// 获取设置文件路径
fn get_settings_file_path() -> AppResult<PathBuf> {
    Ok(get_app_dir()?.join("settings.json"))
}

/// 加载应用设置
pub fn load_settings() -> AppResult<AppSettings> {
    let settings_path = get_settings_file_path()?;
    
    if !settings_path.exists() {
        // 如果设置文件不存在，返回默认设置
        let default_settings = AppSettings::default();
        // 同时保存默认设置到文件
        save_settings(&default_settings)?;
        return Ok(default_settings);
    }

    let content = fs::read_to_string(&settings_path)
        .map_err(|e| AppError::ConfigError(format!("读取设置文件失败: {}", e)))?;

    serde_json::from_str(&content)
        .map_err(|e| AppError::ConfigError(format!("解析设置文件失败: {}", e)))
}

/// 保存应用设置
pub fn save_settings(settings: &AppSettings) -> AppResult<()> {
    let settings_path = get_settings_file_path()?;
    
    // 确保父目录存在
    if let Some(parent) = settings_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| AppError::ConfigError(format!("创建设置目录失败: {}", e)))?;
    }

    let content = serde_json::to_string_pretty(settings)
        .map_err(|e| AppError::ConfigError(format!("序列化设置失败: {}", e)))?;

    fs::write(&settings_path, content)
        .map_err(|e| AppError::ConfigError(format!("写入设置文件失败: {}", e)))?;

    Ok(())
}

/// 获取当前下载路径
pub fn get_download_path() -> AppResult<PathBuf> {
    let settings = load_settings()?;
    let path = PathBuf::from(&settings.download_path);
    
    // 确保目录存在
    if !path.exists() {
        fs::create_dir_all(&path)
            .map_err(|e| AppError::ConfigError(format!("创建下载目录失败: {}", e)))?;
    }
    
    Ok(path)
}

/// 设置下载路径
pub fn set_download_path(path: &str) -> AppResult<()> {
    let mut settings = load_settings()?;
    settings.download_path = path.to_string();
    save_settings(&settings)
}
