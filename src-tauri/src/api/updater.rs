use tauri::Window;

// 重新导出更新服务的函数，方便在main.rs中注册
#[tauri::command]
pub async fn check_update(current_version: String) -> Result<crate::services::updater::UpdateInfo, String> {
    crate::services::updater::check_update(current_version)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn download_and_install_update(window: Window, download_url: String) -> Result<(), String> {
    crate::services::updater::download_and_install_update(window, download_url)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_app_version() -> String {
    crate::services::updater::get_app_version()
} 