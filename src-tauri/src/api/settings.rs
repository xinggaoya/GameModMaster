use crate::api::error::AppResult;
use crate::services::settings::{self, AppSettings};
use tauri_plugin_dialog::DialogExt;

/// 获取应用设置
#[tauri::command]
pub fn get_settings() -> AppResult<AppSettings> {
    settings::load_settings()
}

/// 保存应用设置
#[tauri::command]
pub fn save_settings(settings: AppSettings) -> AppResult<()> {
    settings::save_settings(&settings)
}

/// 获取下载路径
#[tauri::command]
pub fn get_download_path() -> AppResult<String> {
    settings::get_download_path().map(|p| p.to_string_lossy().to_string())
}

/// 设置下载路径
#[tauri::command]
pub fn set_download_path(path: String) -> AppResult<()> {
    settings::set_download_path(&path)
}

/// 选择下载文件夹 (打开文件夹选择对话框)
#[tauri::command]
pub async fn select_download_folder<R: tauri::Runtime>(
    app_handle: tauri::AppHandle<R>,
) -> AppResult<Option<String>> {
    let file_path = app_handle
        .dialog()
        .file()
        .set_title("选择下载目录")
        .blocking_pick_folder();

    match file_path {
        Some(path) => {
            let path_str = path.to_string();
            // 自动保存新路径到设置
            settings::set_download_path(&path_str)?;
            Ok(Some(path_str))
        }
        None => Ok(None),
    }
}

/// 打开下载目录
#[tauri::command]
pub fn open_download_folder() -> AppResult<()> {
    let download_path = settings::get_download_path()?;
    
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        Command::new("explorer")
            .arg(&download_path)
            .spawn()
            .map_err(|e| crate::api::error::AppError::ExecutionError(format!("打开文件夹失败: {}", e)))?;
    }

    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        Command::new("open")
            .arg(&download_path)
            .spawn()
            .map_err(|e| crate::api::error::AppError::ExecutionError(format!("打开文件夹失败: {}", e)))?;
    }

    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        Command::new("xdg-open")
            .arg(&download_path)
            .spawn()
            .map_err(|e| crate::api::error::AppError::ExecutionError(format!("打开文件夹失败: {}", e)))?;
    }

    Ok(())
}
