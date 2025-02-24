use crate::models::trainer::Trainer;
use crate::services::trainer as trainer_service;
use crate::api::error::AppResult;
use std::path::PathBuf;
use serde::Serialize;

// 分页响应结构体
#[derive(Serialize)]
pub struct PaginatedResponse<T> {
    pub trainers: Vec<T>,
    pub total: u32,
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
pub async fn download_trainer(trainer: Trainer) -> AppResult<PathBuf> {
    trainer_service::download_trainer(trainer).await
}

#[tauri::command]
pub fn delete_trainer(trainer_id: String) -> AppResult<()> {
    trainer_service::delete_trainer(trainer_id)
}

#[tauri::command]
pub async fn launch_trainer(trainer_id: String) -> AppResult<()> {
    trainer_service::launch_trainer(trainer_id).await
} 