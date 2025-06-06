// src-tauri/src/commands/config.rs

// 配置相关命令模块，处理应用配置的获取和更新

use log::info;
use crate::models::AppError;
use crate::config::{ConfigManager, AppConfig};

// 获取应用配置命令
#[tauri::command]
pub async fn get_app_config() -> Result<AppConfig, AppError> {
    info!("调用 get_app_config 命令");
    let config_manager = ConfigManager::new()
        .map_err(|e| AppError::ConfigError(format!("创建配置管理器失败: {}", e)))?;
    Ok(config_manager.get_config().clone())
}

// 更新应用配置命令
#[tauri::command]
pub async fn update_app_config(config: AppConfig) -> Result<bool, AppError> {
    info!("调用 update_app_config 命令");
    let mut config_manager = ConfigManager::new()
        .map_err(|e| AppError::ConfigError(format!("创建配置管理器失败: {}", e)))?;
    config_manager.update_config(config)?;
    Ok(true)
} 