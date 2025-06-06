use std::path::{Path, PathBuf};
use crate::models::AppError;

/// 获取文件扩展名
pub fn get_file_extension(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|s| s.to_lowercase())
}

/// 检查文件是否存在
pub fn file_exists(path: &Path) -> bool {
    path.exists() && path.is_file()
}

/// 检查目录是否存在
pub fn dir_exists(path: &Path) -> bool {
    path.exists() && path.is_dir()
}

/// 创建目录（如果不存在）
pub fn ensure_dir_exists(path: &Path) -> Result<(), AppError> {
    if !path.exists() {
        std::fs::create_dir_all(path)
            .map_err(|e| AppError::FileError(format!("创建目录失败: {}", e)))?;
    }
    Ok(())
}

/// 获取文件大小
pub fn get_file_size(path: &Path) -> Result<u64, AppError> {
    let metadata = std::fs::metadata(path)
        .map_err(|e| AppError::FileError(format!("获取文件信息失败: {}", e)))?;
    Ok(metadata.len())
}

/// 格式化文件大小
pub fn format_file_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = size as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{} {}", size as u64, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

/// 生成唯一文件名
pub fn generate_unique_filename(base_path: &Path, extension: &str) -> PathBuf {
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S").to_string();
    let filename = format!("{}_{}.{}", "file", timestamp, extension);
    base_path.join(filename)
}

/// 清理临时文件
pub fn cleanup_temp_files(temp_dir: &Path) -> Result<(), AppError> {
    if temp_dir.exists() {
        std::fs::remove_dir_all(temp_dir)
            .map_err(|e| AppError::FileError(format!("清理临时文件失败: {}", e)))?;
    }
    Ok(())
} 