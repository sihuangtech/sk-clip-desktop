// src-tauri/src/model_manager/storage.rs

// 模型存储管理器，负责管理本地模型文件的存储和清理

use std::path::PathBuf;
use log::info;
use crate::models::AppError;
use super::config::ModelConfigManager;

/// 模型存储管理器
pub struct ModelStorageManager {
    config_manager: ModelConfigManager,
}

impl ModelStorageManager {
    /// 创建新的存储管理器
    pub fn new(config_manager: ModelConfigManager) -> Self {
        Self { config_manager }
    }
    
    /// 获取所有已下载的模型列表
    pub fn list_downloaded_models(&self) -> Result<Vec<ModelInfo>, AppError> {
        let models_dir = self.config_manager.get_models_dir();
        let mut models = Vec::new();
        
        // 扫描Whisper模型
        let whisper_dir = models_dir.join("whisper");
        if whisper_dir.exists() {
            for entry in std::fs::read_dir(&whisper_dir)
                .map_err(|e| AppError::FileError(format!("读取Whisper目录失败: {}", e)))? {
                let entry = entry.map_err(|e| AppError::FileError(format!("读取目录项失败: {}", e)))?;
                let path = entry.path();
                
                if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("pt") {
                    let model_name = path.file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("unknown")
                        .to_string();
                    
                    let size = entry.metadata()
                        .map_err(|e| AppError::FileError(format!("获取文件信息失败: {}", e)))?
                        .len();
                    
                    models.push(ModelInfo {
                        model_type: "whisper".to_string(),
                        model_name,
                        file_path: path,
                        size_bytes: size,
                    });
                }
            }
        }
        
        // TODO: 扫描其他类型的模型
        
        Ok(models)
    }
    
    /// 清理旧的或未使用的模型
    pub fn cleanup_old_models(&self) -> Result<u64, AppError> {
        info!("开始清理旧模型");
        
        let user_config = self.config_manager.get_user_config();
        if !user_config.storage_preferences.auto_cleanup_enabled {
            info!("自动清理已禁用");
            return Ok(0);
        }
        
        let max_storage_bytes = user_config.storage_preferences.max_storage_gb * 1024 * 1024 * 1024;
        let current_usage = self.calculate_total_storage_usage()?;
        
        if current_usage <= max_storage_bytes {
            info!("存储使用量在限制范围内: {} / {} GB", 
                  current_usage / (1024 * 1024 * 1024), 
                  max_storage_bytes / (1024 * 1024 * 1024));
            return Ok(0);
        }
        
        Err(AppError::FileError(format!(
            "模型存储已超出限制，但真实清理策略尚未实现。当前使用 {} 字节，限制 {} 字节。",
            current_usage, max_storage_bytes
        )))
    }
    
    /// 计算总存储使用量
    pub fn calculate_total_storage_usage(&self) -> Result<u64, AppError> {
        let models_dir = self.config_manager.get_models_dir();
        
        fn calculate_dir_size(dir: &PathBuf) -> Result<u64, std::io::Error> {
            let mut size = 0u64;
            
            if dir.is_dir() {
                for entry in std::fs::read_dir(dir)? {
                    let entry = entry?;
                    let path = entry.path();
                    
                    if path.is_dir() {
                        size += calculate_dir_size(&path)?;
                    } else {
                        size += entry.metadata()?.len();
                    }
                }
            }
            
            Ok(size)
        }
        
        let total_size = calculate_dir_size(&models_dir)
            .map_err(|e| AppError::FileError(format!("计算目录大小失败: {}", e)))?;
        
        Ok(total_size)
    }
    
    /// 删除指定模型
    pub fn delete_model(&self, model_type: &str, model_name: &str) -> Result<(), AppError> {
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
        
        std::fs::remove_file(&model_path)
            .map_err(|e| AppError::FileError(format!("删除模型文件失败: {}", e)))?;
        
        info!("已删除模型: {}", model_path.display());
        Ok(())
    }
    
    /// 获取存储统计信息
    pub fn get_storage_stats(&self) -> Result<StorageStats, AppError> {
        let total_usage = self.calculate_total_storage_usage()?;
        let user_config = self.config_manager.get_user_config();
        let max_storage = user_config.storage_preferences.max_storage_gb * 1024 * 1024 * 1024;
        
        let models = self.list_downloaded_models()?;
        let model_count = models.len();
        
        Ok(StorageStats {
            total_usage_bytes: total_usage,
            max_storage_bytes: max_storage,
            model_count,
            usage_percentage: if max_storage > 0 {
                (total_usage as f64 / max_storage as f64 * 100.0) as u32
            } else {
                0
            },
        })
    }
}

/// 模型信息
#[derive(Debug, Clone)]
pub struct ModelInfo {
    pub model_type: String,
    pub model_name: String,
    pub file_path: PathBuf,
    pub size_bytes: u64,
}

/// 存储统计信息
#[derive(Debug, Clone)]
pub struct StorageStats {
    pub total_usage_bytes: u64,
    pub max_storage_bytes: u64,
    pub model_count: usize,
    pub usage_percentage: u32,
} 
