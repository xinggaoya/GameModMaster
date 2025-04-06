use crate::api::error::AppResult;

#[cfg(target_os = "windows")]
use windows_sys::Win32::UI::Shell::ShellExecuteW;

#[cfg(target_os = "windows")]
use windows_sys::Win32::UI::WindowsAndMessaging::SW_SHOW;

#[cfg(target_os = "windows")]
use std::os::windows::ffi::OsStrExt;

#[tauri::command]
pub async fn restart_as_admin() -> AppResult<()> {
    #[cfg(target_os = "windows")]
    {
        // 获取当前可执行文件路径
        let executable_path = std::env::current_exe()?;
        let current_dir = std::env::current_dir()?;
        
        // 转换路径为宽字符
        let wide_path: Vec<u16> = executable_path.as_os_str()
            .encode_wide()
            .chain(Some(0))
            .collect();
        
        // runas操作请求管理员权限
        let wide_operation: Vec<u16> = "runas".encode_utf16().chain(Some(0)).collect();
        
        // 当前目录作为工作目录
        let wide_dir: Vec<u16> = current_dir.as_os_str()
            .encode_wide()
            .chain(Some(0))
            .collect();
        
        let result = unsafe {
            ShellExecuteW(
                0,
                wide_operation.as_ptr(),
                wide_path.as_ptr(),
                std::ptr::null(),
                wide_dir.as_ptr(),
                SW_SHOW,
            )
        };
        
        if result as isize <= 32 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                format!("以管理员权限重启应用失败: {}", result),
            ).into());
        }
        
        // 退出当前进程
        std::process::exit(0);
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "管理员重启功能仅在Windows系统上支持",
        ).into());
    }
} 