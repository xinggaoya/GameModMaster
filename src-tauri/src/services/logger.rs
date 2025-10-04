use crate::api::error::AppResult;
use crate::utils::path::get_app_dir;
use chrono::Local;
use lazy_static::lazy_static;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;

// 日志级别
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warning => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

// 日志配置
pub struct LogConfig {
    #[allow(dead_code)]
    pub max_log_size: u64,    // 最大日志大小（字节）
    pub max_log_files: usize, // 最大日志文件数量
    pub log_dir: PathBuf,     // 日志目录
}

impl Default for LogConfig {
    fn default() -> Self {
        let mut log_dir = get_app_dir().unwrap_or_else(|_| {
            // 如果获取应用目录失败，则使用当前目录
            std::env::current_dir().unwrap_or_else(|_| PathBuf::from("./"))
        });
        log_dir.push("logs");

        Self {
            max_log_size: 5 * 1024 * 1024, // 5MB
            max_log_files: 5,              // 保留5个日志文件
            log_dir,
        }
    }
}

lazy_static! {
    static ref LOGGER: Mutex<Logger> = Mutex::new(Logger::new());
}

// 日志记录器
pub struct Logger {
    config: LogConfig,
    current_log_file: Option<PathBuf>,
}

impl Logger {
    // 创建新的日志记录器
    pub fn new() -> Self {
        let config = LogConfig::default();
        Logger {
            config,
            current_log_file: None,
        }
    }

    // 初始化日志系统
    pub fn init(&mut self) -> AppResult<()> {
        // 确保日志目录存在
        fs::create_dir_all(&self.config.log_dir)?;

        // 检查并清理旧日志
        self.cleanup_logs()?;

        // 创建新的日志文件
        let now = Local::now();
        let log_filename = format!("gm-master-{}.log", now.format("%Y-%m-%d"));
        let log_path = self.config.log_dir.join(&log_filename);

        // 保存当前日志文件路径
        self.current_log_file = Some(log_path);

        // 写入初始日志
        self.log(
            LogLevel::Info,
            &format!(
                "=== 日志初始化时间: {} ===",
                now.format("%Y-%m-%d %H:%M:%S")
            ),
        )?;

        Ok(())
    }

    // 记录日志
    pub fn log(&self, level: LogLevel, message: &str) -> AppResult<()> {
        // 如果没有当前日志文件，跳过
        let log_path = match &self.current_log_file {
            Some(path) => path,
            None => return Ok(()),
        };

        // 获取当前时间
        let now = Local::now();
        let timestamp = now.format("%Y-%m-%d %H:%M:%S%.3f");

        // 格式化日志消息
        let log_entry = format!("[{}] [{}] {}\n", timestamp, level, message);

        // 打开日志文件（追加模式）
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_path)?;

        // 写入日志
        file.write_all(log_entry.as_bytes())?;

        Ok(())
    }

    // 清理旧日志文件
    fn cleanup_logs(&self) -> AppResult<()> {
        // 如果日志目录不存在，创建它
        if !self.config.log_dir.exists() {
            fs::create_dir_all(&self.config.log_dir)?;
            return Ok(());
        }

        // 读取所有日志文件
        let mut log_files = Vec::new();
        for entry in fs::read_dir(&self.config.log_dir)? {
            if let Ok(entry) = entry {
                let path = entry.path();

                // 只处理log文件
                if path.is_file() && path.extension().map_or(false, |ext| ext == "log") {
                    log_files.push(path);
                }
            }
        }

        // 按修改时间排序
        log_files.sort_by(|a, b| {
            let a_modified = fs::metadata(a)
                .and_then(|m| m.modified())
                .unwrap_or_else(|_| std::time::SystemTime::UNIX_EPOCH);
            let b_modified = fs::metadata(b)
                .and_then(|m| m.modified())
                .unwrap_or_else(|_| std::time::SystemTime::UNIX_EPOCH);
            b_modified.cmp(&a_modified) // 降序排列，最新的在前面
        });

        // 删除超过最大数量的旧文件
        if log_files.len() > self.config.max_log_files {
            for old_log in log_files.iter().skip(self.config.max_log_files) {
                if let Err(e) = fs::remove_file(old_log) {
                    println!("无法删除旧日志文件 {}: {}", old_log.display(), e);
                }
            }
        }

        Ok(())
    }
}

// 日志宏，简化日志记录
#[macro_export]
macro_rules! app_log {
    (debug, $($arg:tt)*) => {
        let _ = $crate::services::logger::log(
            $crate::services::logger::LogLevel::Debug,
            &format!($($arg)*)
        );
    };
    (info, $($arg:tt)*) => {
        let _ = $crate::services::logger::log(
            $crate::services::logger::LogLevel::Info,
            &format!($($arg)*)
        );
    };
    (warn, $($arg:tt)*) => {
        let _ = $crate::services::logger::log(
            $crate::services::logger::LogLevel::Warning,
            &format!($($arg)*)
        );
    };
    (error, $($arg:tt)*) => {
        let _ = $crate::services::logger::log(
            $crate::services::logger::LogLevel::Error,
            &format!($($arg)*)
        );
    };
}

// 初始化日志系统
pub fn init_logger() -> AppResult<()> {
    let mut logger = LOGGER.lock().unwrap();
    logger.init()
}

// 记录日志的便捷函数
pub fn log(level: LogLevel, message: &str) -> AppResult<()> {
    let logger = LOGGER.lock().unwrap();
    logger.log(level, message)
}

// 获取日志路径
#[tauri::command]
pub fn get_log_dir() -> AppResult<String> {
    let logger = LOGGER.lock().unwrap();
    Ok(logger.config.log_dir.to_string_lossy().to_string())
}

// 导出最新日志
#[tauri::command]
pub fn export_latest_log() -> AppResult<PathBuf> {
    let logger = LOGGER.lock().unwrap();

    // 获取最新的日志文件
    match &logger.current_log_file {
        Some(log_path) => {
            if log_path.exists() {
                Ok(log_path.clone())
            } else {
                Err(crate::api::error::AppError::NotFoundError(
                    "日志文件不存在".to_string(),
                ))
            }
        }
        None => Err(crate::api::error::AppError::NotFoundError(
            "未找到当前日志文件".to_string(),
        )),
    }
}
