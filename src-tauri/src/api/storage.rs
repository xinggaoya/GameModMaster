use tauri::AppHandle;
use crate::services::storage::StorageService;
use crate::models::trainer::{Trainer, InstalledTrainer};
use anyhow::Result;

// 获取已安装的修改器
#[tauri::command]
pub async fn get_installed_trainers(app: AppHandle) -> Result<Vec<InstalledTrainer>, String> {
    match StorageService::get::<Vec<InstalledTrainer>>(&app, "installedTrainers").await {
        Ok(Some(trainers)) => Ok(trainers),
        Ok(None) => Ok(vec![]),
        Err(e) => Err(e.to_string()),
    }
}

// 保存已安装的修改器
#[tauri::command]
pub async fn save_installed_trainers(app: AppHandle, trainers: Vec<InstalledTrainer>) -> Result<(), String> {
    StorageService::set(&app, "installedTrainers", &trainers).await.map_err(|e| e.to_string())
}

// 获取已下载的修改器
#[tauri::command]
pub async fn get_downloaded_trainers(app: AppHandle) -> Result<Vec<Trainer>, String> {
    match StorageService::get::<Vec<Trainer>>(&app, "downloadedTrainers").await {
        Ok(Some(trainers)) => Ok(trainers),
        Ok(None) => Ok(vec![]),
        Err(e) => Err(e.to_string()),
    }
}

// 保存已下载的修改器
#[tauri::command]
pub async fn save_downloaded_trainers(app: AppHandle, trainers: Vec<Trainer>) -> Result<(), String> {
    StorageService::set(&app, "downloadedTrainers", &trainers).await.map_err(|e| e.to_string())
}

// 缓存修改器列表
#[tauri::command]
pub async fn cache_trainer_list(app: AppHandle, page: u32, trainers: Vec<Trainer>) -> Result<(), String> {
    let key = format!("trainerList_{}", page);
    StorageService::set_cache(&app, &key, &trainers).await.map_err(|e| e.to_string())
}

// 获取缓存的修改器列表
#[tauri::command]
pub async fn get_cached_trainer_list(app: AppHandle, page: u32) -> Result<Option<Vec<Trainer>>, String> {
    let key = format!("trainerList_{}", page);
    StorageService::get_cache(&app, &key).await.map_err(|e| e.to_string())
}

// 缓存搜索结果
#[tauri::command]
pub async fn cache_search_results(app: AppHandle, query: String, page: u32, trainers: Vec<Trainer>) -> Result<(), String> {
    let key = format!("searchResults_{}_{}", query, page);
    StorageService::set_cache(&app, &key, &trainers).await.map_err(|e| e.to_string())
}

// 获取缓存的搜索结果
#[tauri::command]
pub async fn get_cached_search_results(app: AppHandle, query: String, page: u32) -> Result<Option<Vec<Trainer>>, String> {
    let key = format!("searchResults_{}_{}", query, page);
    StorageService::get_cache(&app, &key).await.map_err(|e| e.to_string())
}

// 清理过期缓存
#[tauri::command]
pub async fn clean_expired_cache(app: AppHandle) -> Result<(), String> {
    StorageService::clean_expired_cache(&app).await.map_err(|e| e.to_string())
}

// 从 localStorage 迁移数据
#[tauri::command]
pub async fn migrate_from_local_storage(app: AppHandle, data: serde_json::Value) -> Result<(), String> {
    if let serde_json::Value::Object(map) = data {
        let mut storage_data = std::collections::HashMap::new();
        
        for (key, value) in map {
            storage_data.insert(key, value);
        }
        
        crate::services::storage::migrate_from_local_storage(&app, storage_data)
            .await
            .map_err(|e| e.to_string())
    } else {
        Err("Invalid data format".to_string())
    }
}

// 获取所有存储键
#[tauri::command]
pub async fn get_storage_keys(app: AppHandle) -> Result<Vec<String>, String> {
    StorageService::get_all_keys(&app).await.map_err(|e| e.to_string())
}

// 清空所有存储
#[tauri::command]
pub async fn clear_all_storage(app: AppHandle) -> Result<(), String> {
    StorageService::clear_all(&app).await.map_err(|e| e.to_string())
}