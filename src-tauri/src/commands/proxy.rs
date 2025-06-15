// src-tauri/src/commands/proxy.rs

// 代理相关命令模块，处理代理配置和管理

use log::info;
use serde_json::Value;


use crate::models::AppError;
use crate::model_manager::{ModelConfigManager, ProxyManager};

// 获取代理配置命令
#[tauri::command]
pub async fn get_proxy_config() -> Result<Value, AppError> {
    info!("调用 get_proxy_config 命令");
    
    let config_manager = ModelConfigManager::new()?;
    let models_dir = config_manager.get_models_dir().clone();
    let proxy_manager = ProxyManager::new(models_dir)?;
    
    let current_proxy = proxy_manager.get_current_proxy();
    let proxy_profiles = proxy_manager.get_proxy_profiles();
    
    Ok(serde_json::json!({
        "current_proxy": current_proxy.map(|p| serde_json::json!({
            "type": p.proxy_type.to_string(),
            "host": p.host,
            "port": p.port,
            "has_auth": p.username.is_some()
        })),
        "proxy_profiles": proxy_profiles,
        "user_agent": proxy_manager.get_user_agent()
    }))
}

// 应用代理配置文件命令
#[tauri::command]
pub async fn apply_proxy_profile(profile_name: String) -> Result<String, AppError> {
    info!("调用 apply_proxy_profile 命令，配置文件: {}", profile_name);
    
    let config_manager = ModelConfigManager::new()?;
    let models_dir = config_manager.get_models_dir().clone();
    let mut proxy_manager = ProxyManager::new(models_dir)?;
    
    proxy_manager.apply_proxy_profile(&profile_name)?;
    
    Ok(format!("已应用代理配置文件: {}", profile_name))
}

// 禁用代理命令
#[tauri::command]
pub async fn disable_proxy() -> Result<String, AppError> {
    info!("调用 disable_proxy 命令");
    
    let config_manager = ModelConfigManager::new()?;
    let models_dir = config_manager.get_models_dir().clone();
    let mut proxy_manager = ProxyManager::new(models_dir)?;
    
    proxy_manager.disable_proxy()?;
    
    Ok("代理已禁用".to_string())
}

// 测试代理连接命令
#[tauri::command]
pub async fn test_proxy_connection(
    proxy_type: String,
    host: String,
    port: u16,
    username: Option<String>,
    password: Option<String>
) -> Result<bool, AppError> {
    info!("调用 test_proxy_connection 命令");
    
    let config_manager = ModelConfigManager::new()?;
    let models_dir = config_manager.get_models_dir().clone();
    let proxy_manager = ProxyManager::new(models_dir)?;
    
    let proxy_info = crate::model_manager::ProxyInfo {
        proxy_type: crate::model_manager::ProxyType::from(proxy_type.as_str()),
        host,
        port,
        username,
        password,
    };
    
    proxy_manager.test_proxy(&proxy_info).await
}

// 自动检测代理命令
#[tauri::command]
pub async fn auto_detect_proxy() -> Result<Value, AppError> {
    info!("调用 auto_detect_proxy 命令");
    
    let config_manager = ModelConfigManager::new()?;
    let models_dir = config_manager.get_models_dir().clone();
    let proxy_manager = ProxyManager::new(models_dir)?;
    
    let detected_proxy = proxy_manager.auto_detect_proxy();
    
    Ok(serde_json::json!({
        "detected": detected_proxy.is_some(),
        "proxy": detected_proxy.map(|p| serde_json::json!({
            "type": p.proxy_type.to_string(),
            "host": p.host,
            "port": p.port,
            "url": p.to_url()
        }))
    }))
}

// 获取镜像URL命令
#[tauri::command]
pub async fn get_mirror_url(original_url: String) -> Result<String, AppError> {
    info!("调用 get_mirror_url 命令，原始URL: {}", original_url);
    
    let config_manager = ModelConfigManager::new()?;
    let models_dir = config_manager.get_models_dir().clone();
    let proxy_manager = ProxyManager::new(models_dir)?;
    
    let mirror_url = proxy_manager.get_mirror_url(&original_url);
    
    Ok(mirror_url)
}

// 测试下载连接命令
#[tauri::command]
pub async fn test_download_connection(url: String) -> Result<bool, AppError> {
    info!("调用 test_download_connection 命令，URL: {}", url);
    
    let config_manager = ModelConfigManager::new()?;
    let downloader = crate::model_manager::ModelDownloader::new(config_manager)?;
    
    downloader.test_download_connection(&url).await
} 