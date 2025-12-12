use crate::api::error::AppResult;
use crate::models::trainer::Trainer;
use crate::services::trainer as trainer_service;
use serde::Serialize;
use std::path::PathBuf;
use tauri::Runtime;

// 分页响应结构体
#[derive(Serialize)]
pub struct PaginatedResponse<T> {
    pub trainers: Vec<T>,
    pub total: u32,
}

// 下载进度响应
#[derive(Clone, Serialize)]
pub struct DownloadProgress {
    pub trainer_id: String,
    pub progress: f64, // 0-100
    pub downloaded_bytes: u64,
    pub total_bytes: Option<u64>,
    pub status: String,     // "downloading", "completed", "error"
    pub speed: Option<f64>, // 下载速度 KB/s
}

#[tauri::command]
pub async fn fetch_trainers(page: u32) -> AppResult<PaginatedResponse<Trainer>> {
    trainer_service::fetch_trainers(page).await
}

#[tauri::command]
pub async fn search_trainers(query: String, page: u32) -> AppResult<PaginatedResponse<Trainer>> {
    trainer_service::search_trainers(query, page).await
}

#[tauri::command]
pub async fn get_trainer_detail(id: String) -> AppResult<Trainer> {
    trainer_service::get_trainer_detail(id).await
}

#[tauri::command]
pub async fn download_trainer<R: Runtime>(
    app_handle: tauri::AppHandle<R>,
    trainer: Trainer,
) -> AppResult<PathBuf> {
    trainer_service::download_trainer(app_handle, trainer).await
}

#[tauri::command]
pub async fn delete_trainer(trainer_id: String) -> AppResult<()> {
    trainer_service::delete_trainer(trainer_id).await
}

#[tauri::command]
pub async fn launch_trainer(trainer_id: String) -> AppResult<()> {
    trainer_service::launch_trainer(trainer_id).await
}
