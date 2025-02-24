use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Trainer {
    pub id: String,
    pub name: String,
    pub version: String,
    pub game_version: String,
    pub download_url: String,
    pub description: String,
    pub thumbnail: String,
    pub download_count: i32,
    pub last_update: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainerInstallInfo {
    pub trainer: Trainer,
    pub install_path: String,
    pub install_time: String,
    pub last_launch_time: Option<String>,
} 