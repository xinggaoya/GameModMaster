use thiserror::Error;
use serde::{Serialize, Serializer};
use zip::result::ZipError;

// 错误代码枚举，便于前端进行错误处理
#[derive(Debug, Serialize, Clone, Copy)]
pub enum ErrorCode {
    Network = 1000,
    Parse = 2000,
    IO = 3000,
    Download = 4000,
    Config = 5000,
    Json = 6000,
    Zip = 7000,
    Validation = 8000,
    NotFound = 9000,
    Permission = 10000,
    Execution = 11000,
    Unknown = 99999,
}

// 添加注解抑制未使用变体的警告
#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum AppError {
    #[error("网络请求失败: {0}")]
    RequestError(#[from] reqwest::Error),
    
    #[error("HTML解析失败: {0}")]
    ParseError(String),
    
    #[error("IO操作错误: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("下载错误: {0}")]
    DownloadError(String),
    
    #[error("配置错误: {0}")]
    ConfigError(String),

    #[error("JSON解析错误: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("压缩文件错误: {0}")]
    ZipError(#[from] ZipError),
    
    #[error("验证失败: {0}")]
    ValidationError(String),
    
    #[error("资源未找到: {0}")]
    NotFoundError(String),
    
    #[error("权限错误: {0}")]
    PermissionError(String),
    
    #[error("执行错误: {0}")]
    ExecutionError(String),
    
    #[error("未知错误: {0}")]
    UnknownError(String),
}

impl AppError {
    // 获取错误代码
    pub fn error_code(&self) -> ErrorCode {
        match self {
            AppError::RequestError(_) => ErrorCode::Network,
            AppError::ParseError(_) => ErrorCode::Parse,
            AppError::IoError(_) => ErrorCode::IO,
            AppError::DownloadError(_) => ErrorCode::Download,
            AppError::ConfigError(_) => ErrorCode::Config,
            AppError::JsonError(_) => ErrorCode::Json,
            AppError::ZipError(_) => ErrorCode::Zip,
            AppError::ValidationError(_) => ErrorCode::Validation,
            AppError::NotFoundError(_) => ErrorCode::NotFound,
            AppError::PermissionError(_) => ErrorCode::Permission,
            AppError::ExecutionError(_) => ErrorCode::Execution,
            AppError::UnknownError(_) => ErrorCode::Unknown,
        }
    }
    
    // 获取用户友好的错误信息
    pub fn user_message(&self) -> String {
        match self {
            AppError::RequestError(_) => "网络连接异常，请检查您的网络设置后重试".to_string(),
            AppError::ParseError(_) => "数据解析失败，网站结构可能已更改".to_string(),
            AppError::IoError(_) => "文件读写错误，请检查磁盘空间和权限".to_string(),
            AppError::DownloadError(_) => "下载失败，请稍后重试".to_string(),
            AppError::ConfigError(_) => "配置错误，应用程序设置可能已损坏".to_string(),
            AppError::JsonError(_) => "数据格式错误，无法解析JSON数据".to_string(),
            AppError::ZipError(_) => "压缩文件处理失败，文件可能已损坏".to_string(),
            AppError::ValidationError(msg) => format!("验证失败: {}", msg),
            AppError::NotFoundError(msg) => format!("未找到资源: {}", msg),
            AppError::PermissionError(_) => "权限不足，请以管理员身份运行应用程序".to_string(),
            AppError::ExecutionError(_) => "执行操作失败，请确保系统满足运行要求".to_string(),
            AppError::UnknownError(_) => "发生未知错误，请尝试重启应用程序".to_string(),
        }
    }
}

// 用于序列化的错误响应结构
#[derive(Serialize)]
struct ErrorResponse {
    code: ErrorCode,
    message: String,
    details: String,
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let response = ErrorResponse {
            code: self.error_code(),
            message: self.user_message(),
            details: self.to_string(),
        };
        response.serialize(serializer)
    }
}

pub type AppResult<T> = Result<T, AppError>; 