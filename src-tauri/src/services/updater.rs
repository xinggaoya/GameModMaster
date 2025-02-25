use crate::api::error::{AppError, AppResult};
use crate::utils::path::get_app_dir;
use serde::{Deserialize, Serialize};
use std::os::windows::process::CommandExt;
use std::path::Path;
use tauri::{Window, Emitter};
use std::fs::File;
use std::io::Write;

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
    // 创建下载目录和文件路径
    let app_dir = get_app_dir()?;
    let download_path = Path::new(&app_dir).join("update.exe");

    // 发送开始下载事件
    window
        .emit(
            "update-progress",
            UpdateProgress {
                status: "downloading".to_string(),
                progress: 0,
                message: "开始下载更新...".to_string(),
            },
        )
        .map_err(|e| AppError::ExecutionError(format!("发送事件失败: {}", e)))?;

    // 下载文件
    let client = reqwest::Client::new();
    let mut response = client
        .get(&download_url)
        .send()
        .await
        .map_err(|e| AppError::RequestError(e))?;

    // 获取文件大小
    let total_size = response
        .content_length()
        .unwrap_or(0);

    // 创建文件（使用代码块限制作用域）
    {
        let mut file = File::create(&download_path)
            .map_err(|e| AppError::IoError(e))?;

        let mut downloaded = 0u64;
        let mut last_progress = 0u32;

        // 分块下载文件
        while let Some(chunk) = response
            .chunk()
            .await
            .map_err(|e| AppError::RequestError(e))?
        {
            file.write_all(&chunk)
                .map_err(|e| AppError::IoError(e))?;
            
            downloaded += chunk.len() as u64;
            
            if total_size > 0 {
                let progress = ((downloaded as f64 / total_size as f64) * 100.0) as u32;
                if progress > last_progress {
                    last_progress = progress;
                    window
                        .emit(
                            "update-progress",
                            UpdateProgress {
                                status: "downloading".to_string(),
                                progress,
                                message: format!("正在下载: {}%", progress),
                            },
                        )
                        .ok(); // 忽略发送事件的错误
                }
            }
        }
        
        // 确保文件写入磁盘且句柄关闭
        file.flush().map_err(|e| AppError::IoError(e))?;
        // 文件在这里离开作用域，自动关闭
    }

    // 发送下载完成事件
    window
        .emit(
            "update-progress",
            UpdateProgress {
                status: "completed".to_string(),
                progress: 100,
                message: "下载完成，准备安装...".to_string(),
            },
        )
        .map_err(|e| AppError::ExecutionError(format!("发送事件失败: {}", e)))?;

    // 添加短暂延迟，确保文件操作完全完成
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    // Windows平台特定代码：启动安装程序
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new(&download_path)
            .creation_flags(0x08000000) // 不显示窗口
            .spawn()
            .map_err(|e| AppError::ExecutionError(format!("启动安装程序失败: {}", e)))?;
    }

    // 非Windows平台
    #[cfg(not(target_os = "windows"))]
    {
        std::process::Command::new(&download_path)
            .spawn()
            .map_err(|e| AppError::ExecutionError(format!("启动安装程序失败: {}", e)))?;
    }

    Ok(())
}

// 获取应用当前版本
pub fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
} 