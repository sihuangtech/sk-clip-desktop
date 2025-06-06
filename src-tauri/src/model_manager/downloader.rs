// src-tauri/src/model_manager/downloader.rs

// 模型下载器，负责从网络下载AI模型文件

use std::path::PathBuf;
use log::{info, warn, error};
use crate::models::AppError;
use super::config::{ModelConfigManager, WhisperModelInfo};
use super::proxy::{ProxyManager, ProxyInfo};

/// 模型下载器
pub struct ModelDownloader {
    config_manager: ModelConfigManager,
    proxy_manager: ProxyManager,
}

impl ModelDownloader {
    /// 创建新的模型下载器
    pub fn new(config_manager: ModelConfigManager) -> Result<Self, AppError> {
        let models_dir = config_manager.get_models_dir().clone();
        let proxy_manager = ProxyManager::new(models_dir)?;
        
        Ok(Self { 
            config_manager,
            proxy_manager,
        })
    }
    
    /// 下载Whisper模型
    pub async fn download_whisper_model(&self, model_name: &str) -> Result<PathBuf, AppError> {
        info!("开始下载Whisper模型: {}", model_name);
        
        let model_config = self.config_manager.get_model_config();
        let whisper_model = model_config.models.whisper.models.get(model_name)
            .ok_or_else(|| AppError::ConfigError(format!("未找到模型: {}", model_name)))?;
        
        let models_dir = self.config_manager.get_models_dir();
        let whisper_dir = models_dir.join("whisper");
        let model_path = whisper_dir.join(format!("{}.pt", model_name));
        
        // 检查模型是否已存在
        if model_path.exists() {
            info!("模型已存在: {}", model_path.display());
            return Ok(model_path);
        }
        
        // 获取下载URL，可能使用镜像
        let download_url = self.proxy_manager.get_mirror_url(&whisper_model.download_url);
        info!("下载URL: {}", download_url);
        
        // 检查是否需要使用代理
        let use_proxy = !self.proxy_manager.should_bypass_proxy(&download_url);
        if let Some(proxy_info) = self.proxy_manager.get_current_proxy() {
            if use_proxy {
                info!("使用代理下载: {}", proxy_info.to_url());
                // TODO: 使用代理下载
            } else {
                info!("绕过代理，直接下载");
            }
        }
        
        // 实际下载逻辑
        self.download_file_with_proxy(&download_url, &model_path).await?;
        
        info!("模型下载完成: {}", model_path.display());
        Ok(model_path)
    }
    
    /// 使用代理下载文件
    async fn download_file_with_proxy(&self, url: &str, output_path: &PathBuf) -> Result<(), AppError> {
        info!("开始下载文件: {} -> {}", url, output_path.display());
        
        // 创建目录
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| AppError::FileError(format!("创建目录失败: {}", e)))?;
        }
        
        // TODO: 实现实际的HTTP下载逻辑，支持代理
        // 这里只是模拟下载过程
        let proxy_info = self.proxy_manager.get_current_proxy();
        
        if let Some(proxy) = &proxy_info {
            info!("通过代理下载: {}", proxy.to_url());
            // 模拟代理下载延迟
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        } else {
            info!("直接下载");
            // 模拟直接下载延迟
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        }
        
        // 模拟创建模型文件
        std::fs::write(output_path, b"mock model data with proxy support")
            .map_err(|e| AppError::FileError(format!("创建模型文件失败: {}", e)))?;
        
        Ok(())
    }
    
    /// 测试下载连接
    pub async fn test_download_connection(&self, url: &str) -> Result<bool, AppError> {
        info!("测试下载连接: {}", url);
        
        // 检查是否需要使用代理
        if let Some(proxy_info) = self.proxy_manager.get_current_proxy() {
            if !self.proxy_manager.should_bypass_proxy(url) {
                // 测试代理连接
                return self.proxy_manager.test_proxy(&proxy_info).await;
            }
        }
        
        // TODO: 实现直接连接测试
        // 这里只是模拟测试
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        Ok(true)
    }
    
    /// 获取代理管理器的引用
    pub fn get_proxy_manager(&self) -> &ProxyManager {
        &self.proxy_manager
    }
    
    /// 获取代理管理器的可变引用
    pub fn get_proxy_manager_mut(&mut self) -> &mut ProxyManager {
        &mut self.proxy_manager
    }
    
    /// 检查模型是否存在
    pub fn is_model_downloaded(&self, model_type: &str, model_name: &str) -> bool {
        let models_dir = self.config_manager.get_models_dir();
        let model_path = match model_type {
            "whisper" => models_dir.join("whisper").join(format!("{}.pt", model_name)),
            "translation" => models_dir.join("translation").join(model_name),
            "tts" => models_dir.join("tts").join(model_name),
            _ => return false,
        };
        
        model_path.exists()
    }
    
    /// 获取模型文件大小
    pub fn get_model_size(&self, model_type: &str, model_name: &str) -> Result<u64, AppError> {
        let models_dir = self.config_manager.get_models_dir();
        let model_path = match model_type {
            "whisper" => models_dir.join("whisper").join(format!("{}.pt", model_name)),
            "translation" => models_dir.join("translation").join(model_name),
            "tts" => models_dir.join("tts").join(model_name),
            _ => return Err(AppError::ConfigError("不支持的模型类型".to_string())),
        };
        
        if !model_path.exists() {
            return Err(AppError::FileError("模型文件不存在".to_string()));
        }
        
        let metadata = std::fs::metadata(&model_path)
            .map_err(|e| AppError::FileError(format!("获取文件信息失败: {}", e)))?;
        
        Ok(metadata.len())
    }
} 