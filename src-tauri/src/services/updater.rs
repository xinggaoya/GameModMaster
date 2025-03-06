use crate::api::error::{AppError, AppResult};
use log::info;
use reqwest;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use futures_util::StreamExt;
use tauri::Window;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
use tauri::{Emitter};

// 更新信息结构体
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UpdateInfo {
    pub latest_version: String,
    pub download_url: String,
    pub has_update: bool,
    pub release_notes: String,
}

// 更新进度结构体
#[derive(Serialize, Clone, Debug)]
pub struct UpdateProgress {
    pub status: String,
    pub progress: u32,
    pub message: String,
}

// 发送进度更新的辅助函数
fn send_progress_update(window: &Window, status: &str, progress: u32, message: &str) -> AppResult<()> {
    window
        .emit(
            "update-progress",
            UpdateProgress {
                status: status.to_string(),
                progress,
                message: message.to_string(),
            },
        )
        .map_err(|e| AppError::ExecutionError(format!("发送事件失败: {}", e)))
}

// 检查更新
pub async fn check_update(current_version: String) -> AppResult<UpdateInfo> {
    let client = reqwest::Client::new();

    // 获取最新版本信息（请替换为实际的GitHub仓库地址）
    let releases_url = "https://api.github.com/repos/xinggaoya/GameModMaster/releases/latest";
    
    // 发送请求获取最新版本信息
    let response = client
        .get(releases_url)
        .header("User-Agent", "GameModMaster")
        .send()
        .await
        .map_err(|e| AppError::RequestError(e))?;

    if !response.status().is_success() {
        return Err(AppError::DownloadError(format!(
            "获取版本信息失败，状态码: {}",
            response.status()
        )));
    }

    // 解析JSON响应 - 修复类型错误
    let release: serde_json::Value = response
        .json()
        .await
        .map_err(|e| AppError::RequestError(e))?;

    // 提取版本号（去除v前缀）
    let latest_version = release["tag_name"]
        .as_str()
        .ok_or_else(|| AppError::ParseError("无法获取最新版本号".to_string()))?
        .trim_start_matches('v')
        .to_string();

    // 查找可执行文件资源
    let assets = release["assets"]
        .as_array()
        .ok_or_else(|| AppError::ParseError("无法获取发布资源".to_string()))?;

    let exe_asset = assets
        .iter()
        .find(|asset| {
            asset["name"]
                .as_str()
                .map(|name| name.ends_with(".exe"))
                .unwrap_or(false)
        })
        .ok_or_else(|| AppError::NotFoundError("未找到可执行文件".to_string()))?;

    // 获取下载链接
    let download_url = exe_asset["browser_download_url"]
        .as_str()
        .ok_or_else(|| AppError::ParseError("无法获取下载链接".to_string()))?
        .to_string();

    // 提取发布说明
    let release_notes = release["body"]
        .as_str()
        .unwrap_or("暂无更新说明")
        .to_string();

    // 比较版本号，检查是否有更新
    let has_update = latest_version != current_version;

    Ok(UpdateInfo {
        latest_version,
        download_url,
        has_update,
        release_notes,
    })
}

// 下载并安装更新
pub async fn download_and_install_update(
    window: Window,
    download_url: String,
) -> AppResult<()> {
    // 创建临时目录
    let temp_dir = env::temp_dir().join("game_mod_master_update");
    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir)?;
    }
    fs::create_dir_all(&temp_dir)?;

    // 获取下载文件名
    let file_name = download_url
        .split('/')
        .last()
        .ok_or_else(|| AppError::ParseError("无法解析文件名".to_string()))?;
    let target_path = temp_dir.join(file_name);

    // 发送开始下载通知
    send_progress_update(&window, "downloading", 0, "正在准备下载...")?;

    // 使用代理下载
    let client = reqwest::Client::new();
    
    // 原始URL和代理URL
    let original_url = download_url.clone();
    let proxy_url = format!("https://gh-proxy.com/{}", original_url);
    
    info!("尝试通过代理下载: {}", proxy_url);
    send_progress_update(&window, "downloading", 5, "尝试通过代理下载...")?;
    
    // 先尝试使用代理下载
    let response_result = client.get(&proxy_url).send().await;
    
    // 如果代理下载失败，使用原始URL
    let response = if let Ok(resp) = response_result {
        if resp.status().is_success() {
            info!("使用代理下载成功");
            resp
        } else {
            info!("代理下载失败，尝试直接下载: {}", original_url);
            send_progress_update(&window, "downloading", 5, "代理下载失败，尝试直接下载...")?;
            client.get(&original_url).send().await?
        }
    } else {
        info!("代理下载失败，尝试直接下载: {}", original_url);
        send_progress_update(&window, "downloading", 5, "代理下载失败，尝试直接下载...")?;
        client.get(&original_url).send().await?
    };

    // 获取文件大小
    let total_size = response
        .content_length()
        .ok_or_else(|| AppError::DownloadError("无法获取文件大小".to_string()))?;

    // 创建文件（使用代码块限制作用域）
    {
        let mut file = File::create(&target_path)
            .map_err(|e| AppError::IoError(e))?;

        let mut downloaded = 0;
        let mut stream = response.bytes_stream();

        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result.map_err(|e| AppError::DownloadError(e.to_string()))?;
            file.write_all(&chunk)
                .map_err(|e| AppError::IoError(e))?;

            downloaded += chunk.len() as u64;
            let progress = ((downloaded as f64 / total_size as f64) * 90.0) as u32 + 5; // 5-95%
            
            // 发送下载进度
            send_progress_update(
                &window,
                "downloading",
                progress,
                &format!("下载中... {}%", progress),
            )?;
        }
    }

    // 发送安装开始通知
    send_progress_update(&window, "installing", 95, "正在安装更新...")?;

    // 启动安装程序
    info!("下载完成，启动安装程序: {:?}", target_path);

    // 在Windows上启动安装程序
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new(&target_path)
            .creation_flags(0x08000000) // 不显示窗口
            .spawn()
            .map_err(|e| AppError::ExecutionError(format!("启动安装程序失败: {}", e)))?;
    }

    // 在非Windows平台上启动安装程序
    #[cfg(not(target_os = "windows"))]
    {
        std::process::Command::new(&target_path)
            .spawn()
            .map_err(|e| AppError::ExecutionError(format!("启动安装程序失败: {}", e)))?;
    }

    // 发送完成通知
    send_progress_update(&window, "completed", 100, "更新已完成，请重启应用")?;

    Ok(())
}

// 获取应用当前版本
pub fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
} 