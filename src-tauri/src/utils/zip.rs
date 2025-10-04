use std::fs::File;
use std::io;
use std::path::PathBuf;
use zip::ZipArchive;
use crate::api::error::{AppError, AppResult};

// 验证ZIP文件是否有效
#[allow(dead_code)]
pub fn validate_zip(zip_path: &PathBuf) -> AppResult<()> {
    // 尝试打开文件
    let file = File::open(zip_path)?;

    // 尝试解析为ZIP文件
    match ZipArchive::new(file) {
        Ok(archive) => {
            // 检查是否为空
            if archive.len() == 0 {
                return Err(AppError::ValidationError("ZIP文件为空".to_string()));
            }

            // 验证成功
            Ok(())
        },
        Err(e) => {
            // 检查是否是中央目录结束标记缺失的错误
            let error_str = e.to_string();
            if error_str.contains("central directory") || error_str.contains("end of central directory") {
                return Err(AppError::ZipError(e).with_details("压缩文件错误: invalid Zip archive: Could not find central directory end"));
            }

            // 检查文件大小
            let metadata = std::fs::metadata(zip_path)?;
            if metadata.len() < 100 { // 如果文件小于100字节，可能不是有效的zip文件
                return Err(AppError::ValidationError("文件大小异常，可能不是有效的压缩文件".to_string()));
            }

            // 转换其他错误类型
            Err(AppError::ZipError(e))
        }
    }
}

pub fn extract_zip(zip_path: &PathBuf, extract_dir: &PathBuf) -> AppResult<()> {
    let file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => extract_dir.join(path),
            None => continue,
        };

        if (*file.name()).ends_with('/') {
            std::fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(p)?;
                }
            }
            let mut outfile = File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }

        // 设置文件权限（仅在 Unix 系统）
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                std::fs::set_permissions(&outpath, std::fs::Permissions::from_mode(mode))?;
            }
        }
    }

    Ok(())
}