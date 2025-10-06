use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;
use serde_json::Value;
use std::collections::HashMap;
use anyhow::Result;

// 缓存项结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheItem<T> {
    pub data: T,
    pub timestamp: i64,
    pub expiration: i64,
}

// 存储键名常量
pub const STORE_PATH: &str = "app_data.json";
pub const INSTALLED_TRAINERS_KEY: &str = "installedTrainers";
pub const DOWNLOADED_TRAINERS_KEY: &str = "downloadedTrainers";
pub const TRAINER_LIST_PREFIX: &str = "trainerList_";
pub const SEARCH_RESULTS_PREFIX: &str = "searchResults_";

// 缓存配置
const CACHE_EXPIRATION_TIME: i64 = 1000 * 60 * 15; // 15分钟

pub struct StorageService;

impl StorageService {
    // 保存任意数据到存储
    pub async fn set<T: Serialize>(app: &AppHandle, key: &str, value: &T) -> Result<()> {
        let store = app.store(STORE_PATH)?;
        let json_value = serde_json::to_value(value)?;
        store.set(key.to_string(), json_value);
        store.save()?;
        Ok(())
    }

    // 从存储获取数据
    pub async fn get<T: for<'de> Deserialize<'de>>(app: &AppHandle, key: &str) -> Result<Option<T>> {
        let store = app.store(STORE_PATH)?;
        if let Some(value) = store.get(key) {
            let typed_value: T = serde_json::from_value(value)?;
            Ok(Some(typed_value))
        } else {
            Ok(None)
        }
    }

    // 删除存储项
    pub async fn delete(app: &AppHandle, key: &str) -> Result<()> {
        let store = app.store(STORE_PATH)?;
        store.delete(key);
        store.save()?;
        Ok(())
    }

    // 保存缓存项
    pub async fn set_cache<T: Serialize>(app: &AppHandle, key: &str, data: &T) -> Result<()> {
        let now = chrono::Utc::now().timestamp_millis();
        let cache_item = CacheItem {
            data: serde_json::to_value(data)?,
            timestamp: now,
            expiration: now + CACHE_EXPIRATION_TIME,
        };
        
        let store = app.store(STORE_PATH)?;
        store.set(key.to_string(), serde_json::to_value(&cache_item)?);
        store.save()?;
        Ok(())
    }

    // 获取缓存项（检查过期）
    pub async fn get_cache<T: for<'de> Deserialize<'de>>(app: &AppHandle, key: &str) -> Result<Option<T>> {
        let store = app.store(STORE_PATH)?;
        if let Some(value) = store.get(key) {
            let cache_item: CacheItem<Value> = serde_json::from_value(value)?;
            
            // 检查是否过期
            let now = chrono::Utc::now().timestamp_millis();
            if cache_item.expiration < now {
                // 过期了，删除缓存
                store.delete(key);
                store.save()?;
                return Ok(None);
            }

            // 反序列化数据
            let typed_data: T = serde_json::from_value(cache_item.data)?;
            Ok(Some(typed_data))
        } else {
            Ok(None)
        }
    }

    // 清理过期缓存
    pub async fn clean_expired_cache(app: &AppHandle) -> Result<()> {
        let store = app.store(STORE_PATH)?;
        let now = chrono::Utc::now().timestamp_millis();
        let mut keys_to_remove = Vec::new();

        // 获取所有键并检查过期
        let keys = store.keys();
        for key in keys {
            if key.starts_with(TRAINER_LIST_PREFIX) || key.starts_with(SEARCH_RESULTS_PREFIX) {
                if let Some(value) = store.get(&key) {
                    if let Ok(cache_item) = serde_json::from_value::<CacheItem<Value>>(value) {
                        if cache_item.expiration < now {
                            keys_to_remove.push(key);
                        }
                    } else {
                        // 无效的缓存项，也删除
                        keys_to_remove.push(key);
                    }
                }
            }
        }

        // 删除过期的键
        for key in &keys_to_remove {
            store.delete(key);
        }

        if !keys_to_remove.is_empty() {
            store.save()?;
        }

        Ok(())
    }

    // 获取所有存储的键
    pub async fn get_all_keys(app: &AppHandle) -> Result<Vec<String>> {
        let store = app.store(STORE_PATH)?;
        Ok(store.keys())
    }

    // 清空所有数据
    pub async fn clear_all(app: &AppHandle) -> Result<()> {
        let store = app.store(STORE_PATH)?;
        store.clear();
        store.save()?;
        Ok(())
    }
}

// 从 localStorage 迁移数据的辅助函数
pub async fn migrate_from_local_storage(app: &AppHandle, local_data: HashMap<String, Value>) -> Result<()> {
    for (key, value) in local_data {
        StorageService::set(app, &key, &value).await?;
    }
    Ok(())
}