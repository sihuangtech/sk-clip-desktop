// src-tauri/src/file_manager.rs

// 该文件负责处理应用程序的文件系统相关的操作

use std::path::PathBuf;
use std::fs;
use log::{info, error};
use anyhow::Result;
use dirs;

// 导入自定义错误类型
use crate::models::AppError;

// 获取应用数据目录
// 返回应用程序存储数据、配置和输出文件的目录路径。
// 如果目录不存在，会自动创建。
// 返回值: Result 包含 PathBuf (成功时的目录路径) 或 AppError (失败时的错误)
pub fn get_app_data_dir() -> Result<PathBuf, AppError> {
    info!("获取应用数据目录...");
    dirs::data_dir()
        .ok_or_else(|| {
            error!("无法获取应用数据目录");
            AppError::DirectoryError("无法获取应用数据目录".to_string())
        })
}

// 复制文件
// 将源文件复制到目标文件。
// 参数: source_path - 源文件的路径
//       target_path - 目标文件的路径
// 返回值: Result 包含 () (成功) 或 AppError (失败)
pub fn copy_file(source_path: &PathBuf, target_path: &PathBuf) -> Result<(), AppError> {
    info!("复制文件: {} -> {}", source_path.display(), target_path.display());
    fs::copy(source_path, target_path)
        .map_err(|e| {
            error!("复制文件失败: {} -> {}: {}", source_path.display(), target_path.display(), e);
            AppError::FileError(e.to_string())
        })?;
    info!("文件复制成功");
    Ok(())
}

// 删除文件
// 删除指定路径的文件。
// 参数: target_path - 要删除的文件的路径
// 返回值: Result 包含 () (成功) 或 AppError (失败)
pub fn delete_file(target_path: &PathBuf) -> Result<(), AppError> {
    info!("删除文件: {}", target_path.display());
    // 检查文件是否存在，如果不存在则认为删除成功
    if !target_path.exists() {
        info!("文件 {} 不存在，无需删除", target_path.display());
        return Ok(());
    }
    
    // 删除文件
    fs::remove_file(target_path)
        .map_err(|e| {
            error!("删除文件失败: {}: {}", target_path.display(), e);
            AppError::FileError(e.to_string())
        })?;
    info!("文件删除成功: {}", target_path.display());
    Ok(())
}

// TODO: 添加其他文件管理相关的函数，例如：
// - 创建目录
// - 读取文件内容
// - 写入文件内容
// - 列出目录内容
// - 清理临时文件等 