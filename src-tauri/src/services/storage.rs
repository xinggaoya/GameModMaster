use std::collections::HashMap;
use std::path::PathBuf;

use anyhow::{Context, Result};
use chrono::Utc;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::trainer::{InstalledTrainer, Trainer};
use crate::utils::path::get_app_dir;

// 存储键名常量（用于前后端约定）
pub const INSTALLED_TRAINERS_KEY: &str = "installedTrainers";
pub const DOWNLOADED_TRAINERS_KEY: &str = "downloadedTrainers";
pub const TRAINER_LIST_PREFIX: &str = "trainerList_";
pub const SEARCH_RESULTS_PREFIX: &str = "searchResults_";

// 缓存配置
const CACHE_EXPIRATION_TIME: i64 = 1000 * 60 * 15; // 15分钟
const DB_FILE_NAME: &str = "app.db";

// 缓存项结构（兼容 localStorage 迁移数据）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheItem<T> {
    pub data: T,
    pub timestamp: i64,
    pub expiration: i64,
}

fn get_db_path() -> Result<PathBuf> {
    Ok(get_app_dir()?.join(DB_FILE_NAME))
}

async fn with_conn<T, F>(f: F) -> Result<T>
where
    T: Send + 'static,
    F: FnOnce(&mut Connection) -> Result<T> + Send + 'static,
{
    let db_path = get_db_path()?;
    tauri::async_runtime::spawn_blocking(move || {
        let mut conn = Connection::open(db_path)?;
        conn.busy_timeout(std::time::Duration::from_secs(5))?;
        f(&mut conn)
    })
    .await
    .context("数据库线程池执行失败")?
}

pub async fn init_db() -> Result<()> {
    let db_path = get_db_path()?;
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let path_clone = db_path.clone();
    tauri::async_runtime::spawn_blocking(move || {
        let conn = Connection::open(path_clone)?;
        conn.execute_batch(
            "
            PRAGMA journal_mode=WAL;
            PRAGMA synchronous=NORMAL;
            CREATE TABLE IF NOT EXISTS installed_trainers (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                version TEXT NOT NULL,
                game_version TEXT NOT NULL,
                download_url TEXT NOT NULL,
                description TEXT NOT NULL,
                thumbnail TEXT NOT NULL,
                download_count INTEGER NOT NULL,
                last_update TEXT NOT NULL,
                installed_path TEXT NOT NULL,
                install_time TEXT NOT NULL,
                last_launch_time TEXT
            );
            CREATE TABLE IF NOT EXISTS downloaded_trainers (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                version TEXT NOT NULL,
                game_version TEXT NOT NULL,
                download_url TEXT NOT NULL,
                description TEXT NOT NULL,
                thumbnail TEXT NOT NULL,
                download_count INTEGER NOT NULL,
                last_update TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS trainer_cache (
                page INTEGER PRIMARY KEY,
                data TEXT NOT NULL,
                expiration INTEGER NOT NULL
            );
            CREATE TABLE IF NOT EXISTS search_cache (
                query TEXT NOT NULL,
                page INTEGER NOT NULL,
                data TEXT NOT NULL,
                expiration INTEGER NOT NULL,
                PRIMARY KEY (query, page)
            );
            ",
        )?;
        Ok::<(), rusqlite::Error>(())
    })
    .await
    .context("初始化数据库失败")??;

    Ok(())
}

pub async fn save_installed_trainers(trainers: Vec<InstalledTrainer>) -> Result<()> {
    with_conn(move |conn| {
        let tx = conn.transaction()?;
        tx.execute("DELETE FROM installed_trainers", [])?;

        {
            let mut stmt = tx.prepare(
                "
                INSERT INTO installed_trainers (
                    id, name, version, game_version, download_url,
                    description, thumbnail, download_count, last_update,
                    installed_path, install_time, last_launch_time
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)
                ",
            )?;

            for t in trainers {
                stmt.execute(params![
                    t.id,
                    t.name,
                    t.version,
                    t.game_version,
                    t.download_url,
                    t.description,
                    t.thumbnail,
                    t.download_count,
                    t.last_update,
                    t.installed_path,
                    t.install_time,
                    t.last_launch_time,
                ])?;
            }
        }

        tx.commit()?;
        Ok(())
    })
    .await
}

pub async fn get_installed_trainers() -> Result<Vec<InstalledTrainer>> {
    with_conn(move |conn| {
        let mut stmt = conn.prepare(
            "
            SELECT id, name, version, game_version, download_url,
                   description, thumbnail, download_count, last_update,
                   installed_path, install_time, last_launch_time
            FROM installed_trainers
            ",
        )?;

        let rows = stmt.query_map([], |row| {
            Ok(InstalledTrainer {
                id: row.get(0)?,
                name: row.get(1)?,
                version: row.get(2)?,
                game_version: row.get(3)?,
                download_url: row.get(4)?,
                description: row.get(5)?,
                thumbnail: row.get(6)?,
                download_count: row.get(7)?,
                last_update: row.get(8)?,
                installed_path: row.get(9)?,
                install_time: row.get(10)?,
                last_launch_time: row.get::<_, Option<String>>(11)?.unwrap_or_default(),
            })
        })?;

        let mut result = Vec::new();
        for trainer in rows {
            result.push(trainer?);
        }
        Ok(result)
    })
    .await
}

pub async fn save_downloaded_trainers(trainers: Vec<Trainer>) -> Result<()> {
    with_conn(move |conn| {
        let tx = conn.transaction()?;
        tx.execute("DELETE FROM downloaded_trainers", [])?;

        {
            let mut stmt = tx.prepare(
                "
                INSERT INTO downloaded_trainers (
                    id, name, version, game_version, download_url,
                    description, thumbnail, download_count, last_update
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
                ",
            )?;

            for t in trainers {
                stmt.execute(params![
                    t.id,
                    t.name,
                    t.version,
                    t.game_version,
                    t.download_url,
                    t.description,
                    t.thumbnail,
                    t.download_count,
                    t.last_update
                ])?;
            }
        }

        tx.commit()?;
        Ok(())
    })
    .await
}

pub async fn get_downloaded_trainers() -> Result<Vec<Trainer>> {
    with_conn(move |conn| {
        let mut stmt = conn.prepare(
            "
            SELECT id, name, version, game_version, download_url,
                   description, thumbnail, download_count, last_update
            FROM downloaded_trainers
            ",
        )?;

        let rows = stmt.query_map([], |row| {
            Ok(Trainer {
                id: row.get(0)?,
                name: row.get(1)?,
                version: row.get(2)?,
                game_version: row.get(3)?,
                download_url: row.get(4)?,
                description: row.get(5)?,
                thumbnail: row.get(6)?,
                download_count: row.get(7)?,
                last_update: row.get(8)?,
            })
        })?;

        let mut result = Vec::new();
        for trainer in rows {
            result.push(trainer?);
        }
        Ok(result)
    })
    .await
}

pub async fn upsert_downloaded_trainer(trainer: Trainer) -> Result<()> {
    with_conn(move |conn| {
        conn.execute(
            "
            INSERT INTO downloaded_trainers (
                id, name, version, game_version, download_url,
                description, thumbnail, download_count, last_update
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
            ON CONFLICT(id) DO UPDATE SET
                name = excluded.name,
                version = excluded.version,
                game_version = excluded.game_version,
                download_url = excluded.download_url,
                description = excluded.description,
                thumbnail = excluded.thumbnail,
                download_count = excluded.download_count,
                last_update = excluded.last_update
            ",
            params![
                trainer.id,
                trainer.name,
                trainer.version,
                trainer.game_version,
                trainer.download_url,
                trainer.description,
                trainer.thumbnail,
                trainer.download_count,
                trainer.last_update,
            ],
        )?;
        Ok(())
    })
    .await
}

pub async fn remove_downloaded_trainer(trainer_id: &str) -> Result<()> {
    let id = trainer_id.to_string();
    with_conn(move |conn| {
        conn.execute("DELETE FROM downloaded_trainers WHERE id = ?1", params![id])?;
        Ok(())
    })
    .await
}

pub async fn upsert_installed_trainer(trainer: InstalledTrainer) -> Result<()> {
    with_conn(move |conn| {
        conn.execute(
            "
            INSERT INTO installed_trainers (
                id, name, version, game_version, download_url,
                description, thumbnail, download_count, last_update,
                installed_path, install_time, last_launch_time
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)
            ON CONFLICT(id) DO UPDATE SET
                name = excluded.name,
                version = excluded.version,
                game_version = excluded.game_version,
                download_url = excluded.download_url,
                description = excluded.description,
                thumbnail = excluded.thumbnail,
                download_count = excluded.download_count,
                last_update = excluded.last_update,
                installed_path = excluded.installed_path,
                install_time = excluded.install_time,
                last_launch_time = excluded.last_launch_time
            ",
            params![
                trainer.id,
                trainer.name,
                trainer.version,
                trainer.game_version,
                trainer.download_url,
                trainer.description,
                trainer.thumbnail,
                trainer.download_count,
                trainer.last_update,
                trainer.installed_path,
                trainer.install_time,
                trainer.last_launch_time
            ],
        )?;
        Ok(())
    })
    .await
}

pub async fn get_installed_trainer_by_id(id: &str) -> Result<Option<InstalledTrainer>> {
    let trainer_id = id.to_string();
    with_conn(move |conn| {
        let mut stmt = conn.prepare(
            "
            SELECT id, name, version, game_version, download_url,
                   description, thumbnail, download_count, last_update,
                   installed_path, install_time, last_launch_time
            FROM installed_trainers WHERE id = ?1
            ",
        )?;
        let mut rows = stmt.query(params![trainer_id])?;
        if let Some(row) = rows.next()? {
            let trainer = InstalledTrainer {
                id: row.get(0)?,
                name: row.get(1)?,
                version: row.get(2)?,
                game_version: row.get(3)?,
                download_url: row.get(4)?,
                description: row.get(5)?,
                thumbnail: row.get(6)?,
                download_count: row.get(7)?,
                last_update: row.get(8)?,
                installed_path: row.get(9)?,
                install_time: row.get(10)?,
                last_launch_time: row.get::<_, Option<String>>(11)?.unwrap_or_default(),
            };
            Ok(Some(trainer))
        } else {
            Ok(None)
        }
    })
    .await
}

pub async fn remove_installed_trainer(id: &str) -> Result<()> {
    let trainer_id = id.to_string();
    with_conn(move |conn| {
        conn.execute("DELETE FROM installed_trainers WHERE id = ?1", params![trainer_id])?;
        Ok(())
    })
    .await
}

pub async fn update_last_launch_time(id: &str, timestamp: &str) -> Result<()> {
    let trainer_id = id.to_string();
    let ts = timestamp.to_string();
    with_conn(move |conn| {
        conn.execute(
            "UPDATE installed_trainers SET last_launch_time = ?1 WHERE id = ?2",
            params![ts, trainer_id],
        )?;
        Ok(())
    })
    .await
}

pub async fn cache_trainer_list(page: u32, trainers: Vec<Trainer>) -> Result<()> {
    let expiration = Utc::now().timestamp_millis() + CACHE_EXPIRATION_TIME;
    let data = serde_json::to_string(&trainers)?;

    with_conn(move |conn| {
        conn.execute(
            "
            INSERT INTO trainer_cache (page, data, expiration)
            VALUES (?1, ?2, ?3)
            ON CONFLICT(page) DO UPDATE SET
                data = excluded.data,
                expiration = excluded.expiration
            ",
            params![page, data, expiration],
        )?;
        Ok(())
    })
    .await
}

pub async fn get_cached_trainer_list(page: u32) -> Result<Option<Vec<Trainer>>> {
    let now = Utc::now().timestamp_millis();
    with_conn(move |conn| {
        let mut stmt = conn.prepare(
            "
            SELECT data, expiration FROM trainer_cache WHERE page = ?1
            ",
        )?;
        let mut rows = stmt.query(params![page])?;
        if let Some(row) = rows.next()? {
            let expiration: i64 = row.get(1)?;
            if expiration < now {
                conn.execute("DELETE FROM trainer_cache WHERE page = ?1", params![page])?;
                return Ok(None);
            }
            let data: String = row.get(0)?;
            let trainers: Vec<Trainer> = serde_json::from_str(&data)?;
            Ok(Some(trainers))
        } else {
            Ok(None)
        }
    })
    .await
}

pub async fn cache_search_results(query: String, page: u32, trainers: Vec<Trainer>) -> Result<()> {
    let expiration = Utc::now().timestamp_millis() + CACHE_EXPIRATION_TIME;
    let data = serde_json::to_string(&trainers)?;

    with_conn(move |conn| {
        conn.execute(
            "
            INSERT INTO search_cache (query, page, data, expiration)
            VALUES (?1, ?2, ?3, ?4)
            ON CONFLICT(query, page) DO UPDATE SET
                data = excluded.data,
                expiration = excluded.expiration
            ",
            params![query, page, data, expiration],
        )?;
        Ok(())
    })
    .await
}

pub async fn get_cached_search_results(query: String, page: u32) -> Result<Option<Vec<Trainer>>> {
    let now = Utc::now().timestamp_millis();
    with_conn(move |conn| {
        let mut stmt = conn.prepare(
            "
            SELECT data, expiration FROM search_cache
            WHERE query = ?1 AND page = ?2
            ",
        )?;
        let mut rows = stmt.query(params![query, page])?;
        if let Some(row) = rows.next()? {
            let expiration: i64 = row.get(1)?;
            if expiration < now {
                conn.execute(
                    "DELETE FROM search_cache WHERE query = ?1 AND page = ?2",
                    params![query, page],
                )?;
                return Ok(None);
            }
            let data: String = row.get(0)?;
            let trainers: Vec<Trainer> = serde_json::from_str(&data)?;
            Ok(Some(trainers))
        } else {
            Ok(None)
        }
    })
    .await
}

pub async fn clean_expired_cache() -> Result<()> {
    let now = Utc::now().timestamp_millis();
    with_conn(move |conn| {
        conn.execute(
            "DELETE FROM trainer_cache WHERE expiration < ?1",
            params![now],
        )?;
        conn.execute(
            "DELETE FROM search_cache WHERE expiration < ?1",
            params![now],
        )?;
        Ok(())
    })
    .await
}

pub async fn get_all_keys() -> Result<Vec<String>> {
    with_conn(move |conn| {
        let mut keys = Vec::new();

        let installed_count: i64 =
            conn.query_row("SELECT COUNT(*) FROM installed_trainers", [], |row| row.get(0))?;
        if installed_count > 0 {
            keys.push(INSTALLED_TRAINERS_KEY.to_string());
        }

        let downloaded_count: i64 =
            conn.query_row("SELECT COUNT(*) FROM downloaded_trainers", [], |row| row.get(0))?;
        if downloaded_count > 0 {
            keys.push(DOWNLOADED_TRAINERS_KEY.to_string());
        }

        let mut trainer_stmt = conn.prepare("SELECT page FROM trainer_cache")?;
        let trainer_iter = trainer_stmt.query_map([], |row| row.get::<_, i64>(0))?;
        for page in trainer_iter {
            let page = page?;
            keys.push(format!("{}{}", TRAINER_LIST_PREFIX, page));
        }

        let mut search_stmt = conn.prepare("SELECT query, page FROM search_cache")?;
        let search_iter =
            search_stmt.query_map([], |row| Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?)))?;
        for item in search_iter {
            let (query, page) = item?;
            keys.push(format!("{}{}_{}", SEARCH_RESULTS_PREFIX, query, page));
        }

        Ok(keys)
    })
    .await
}

pub async fn clear_all() -> Result<()> {
    with_conn(move |conn| {
        conn.execute("DELETE FROM installed_trainers", [])?;
        conn.execute("DELETE FROM downloaded_trainers", [])?;
        conn.execute("DELETE FROM trainer_cache", [])?;
        conn.execute("DELETE FROM search_cache", [])?;
        Ok(())
    })
    .await
}

pub async fn migrate_from_local_storage(local_data: HashMap<String, Value>) -> Result<()> {
    let mut installed: Option<Vec<InstalledTrainer>> = None;
    let mut downloaded: Option<Vec<Trainer>> = None;
    let mut trainer_cache_items: Vec<(u32, CacheItem<Vec<Trainer>>)> = Vec::new();
    let mut search_cache_items: Vec<(String, u32, CacheItem<Vec<Trainer>>)> = Vec::new();

    for (key, value) in local_data {
        if key == INSTALLED_TRAINERS_KEY {
            if let Ok(items) = serde_json::from_value::<Vec<InstalledTrainer>>(value.clone()) {
                installed = Some(items);
            }
        } else if key == DOWNLOADED_TRAINERS_KEY {
            if let Ok(items) = serde_json::from_value::<Vec<Trainer>>(value.clone()) {
                downloaded = Some(items);
            }
        } else if key.starts_with(TRAINER_LIST_PREFIX) {
            if let Some(page_str) = key.strip_prefix(TRAINER_LIST_PREFIX) {
                if let Ok(page) = page_str.parse::<u32>() {
                    if let Ok(cache_item) =
                        serde_json::from_value::<CacheItem<Vec<Trainer>>>(value.clone())
                    {
                        if cache_item.expiration > Utc::now().timestamp_millis() {
                            trainer_cache_items.push((page, cache_item));
                        }
                    }
                }
            }
        } else if key.starts_with(SEARCH_RESULTS_PREFIX) {
            if let Some(rest) = key.strip_prefix(SEARCH_RESULTS_PREFIX) {
                if let Some((query_part, page_part)) = rest.rsplit_once('_') {
                    if let Ok(page) = page_part.parse::<u32>() {
                        let query = query_part.trim_start_matches('_').to_string();
                        if let Ok(cache_item) =
                            serde_json::from_value::<CacheItem<Vec<Trainer>>>(value.clone())
                        {
                            if cache_item.expiration > Utc::now().timestamp_millis() {
                                search_cache_items.push((query, page, cache_item));
                            }
                        }
                    }
                }
            }
        }
    }

    if let Some(items) = installed {
        save_installed_trainers(items).await?;
    }
    if let Some(items) = downloaded {
        save_downloaded_trainers(items).await?;
    }

    for (page, cache_item) in trainer_cache_items {
        let expiration = cache_item.expiration;
        let data = serde_json::to_string(&cache_item.data)?;
        let _ = with_conn(move |conn| {
            conn.execute(
                "
                INSERT INTO trainer_cache (page, data, expiration)
                VALUES (?1, ?2, ?3)
                ON CONFLICT(page) DO UPDATE SET
                    data = excluded.data,
                    expiration = excluded.expiration
                ",
                params![page, data, expiration],
            )?;
            Ok(())
        })
        .await?;
    }

    for (query, page, cache_item) in search_cache_items {
        let expiration = cache_item.expiration;
        let data = serde_json::to_string(&cache_item.data)?;
        let _ = with_conn(move |conn| {
            conn.execute(
                "
                INSERT INTO search_cache (query, page, data, expiration)
                VALUES (?1, ?2, ?3, ?4)
                ON CONFLICT(query, page) DO UPDATE SET
                    data = excluded.data,
                    expiration = excluded.expiration
                ",
                params![query, page, data, expiration],
            )?;
            Ok(())
        })
        .await?;
    }

    Ok(())
}
