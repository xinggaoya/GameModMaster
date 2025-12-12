use crate::models::trainer::{Trainer, InstalledTrainer};
use crate::services::storage;
use anyhow::Result;

// 获取已安装的修改器
#[tauri::command]
pub async fn get_installed_trainers() -> Result<Vec<InstalledTrainer>, String> {
    storage::get_installed_trainers()
        .await
        .map_err(|e| e.to_string())
}

// 保存已安装的修改器
#[tauri::command]
pub async fn save_installed_trainers(trainers: Vec<InstalledTrainer>) -> Result<(), String> {
    storage::save_installed_trainers(trainers)
        .await
        .map_err(|e| e.to_string())
}

// 获取已下载的修改器
#[tauri::command]
pub async fn get_downloaded_trainers() -> Result<Vec<Trainer>, String> {
    storage::get_downloaded_trainers()
        .await
        .map_err(|e| e.to_string())
}

// 保存已下载的修改器
#[tauri::command]
pub async fn save_downloaded_trainers(trainers: Vec<Trainer>) -> Result<(), String> {
    storage::save_downloaded_trainers(trainers)
        .await
        .map_err(|e| e.to_string())
}

// 缓存修改器列表
#[tauri::command]
pub async fn cache_trainer_list(page: u32, trainers: Vec<Trainer>) -> Result<(), String> {
    storage::cache_trainer_list(page, trainers)
        .await
        .map_err(|e| e.to_string())
}

// 获取缓存的修改器列表
#[tauri::command]
pub async fn get_cached_trainer_list(page: u32) -> Result<Option<Vec<Trainer>>, String> {
    storage::get_cached_trainer_list(page)
        .await
        .map_err(|e| e.to_string())
}

// 缓存搜索结果
#[tauri::command]
pub async fn cache_search_results(query: String, page: u32, trainers: Vec<Trainer>) -> Result<(), String> {
    storage::cache_search_results(query, page, trainers)
        .await
        .map_err(|e| e.to_string())
}

// 获取缓存的搜索结果
#[tauri::command]
pub async fn get_cached_search_results(query: String, page: u32) -> Result<Option<Vec<Trainer>>, String> {
    storage::get_cached_search_results(query, page)
        .await
        .map_err(|e| e.to_string())
}

// 清理过期缓存
#[tauri::command]
pub async fn clean_expired_cache() -> Result<(), String> {
    storage::clean_expired_cache()
        .await
        .map_err(|e| e.to_string())
}

// 从 localStorage 迁移数据
#[tauri::command]
pub async fn migrate_from_local_storage(data: serde_json::Value) -> Result<(), String> {
    if let serde_json::Value::Object(map) = data {
        let mut storage_data = std::collections::HashMap::new();
        
        for (key, value) in map {
            storage_data.insert(key, value);
        }
        
        crate::services::storage::migrate_from_local_storage(storage_data)
            .await
            .map_err(|e| e.to_string())
    } else {
        Err("Invalid data format".to_string())
    }
}

// 获取所有存储键
#[tauri::command]
pub async fn get_storage_keys() -> Result<Vec<String>, String> {
    storage::get_all_keys()
        .await
        .map_err(|e| e.to_string())
}

// 清空所有存储
#[tauri::command]
pub async fn clear_all_storage() -> Result<(), String> {
    storage::clear_all()
        .await
        .map_err(|e| e.to_string())
}
