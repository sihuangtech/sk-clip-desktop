// src-tauri/src/ai/whisper/model.rs

use std::path::PathBuf;
use log::info;
use crate::models::AppError;

/// Whisper模型实现
pub struct WhisperModel {
    model_path: PathBuf,
    initialized: bool,
}

impl WhisperModel {
    pub fn new() -> Self {
        Self {
            model_path: PathBuf::new(),
            initialized: false,
        }
    }

    pub async fn initialize(&mut self) -> Result<(), AppError> {
        info!("正在初始化Whisper模型...");
        
        // 获取模型存储目录
        let models_dir = dirs::data_dir()
            .ok_or_else(|| AppError::FileError("无法获取数据目录".to_string()))?
            .join("multisay")
            .join("models");
        
        std::fs::create_dir_all(&models_dir).map_err(|e| {
            AppError::FileError(format!("创建模型目录失败: {}", e))
        })?;

        self.model_path = models_dir.join("whisper-base.bin");
        
        // 检查模型是否已存在
        if !self.model_path.exists() {
            info!("Whisper模型不存在，开始下载...");
            self.download_model().await?;
        }

        self.initialized = true;
        info!("Whisper模型初始化完成");
        Ok(())
    }

    async fn download_model(&self) -> Result<(), AppError> {
        Err(AppError::ModelInitializationError(format!(
            "Whisper 模型不存在，且当前 WhisperModel 未接入真实下载器: {}",
            self.model_path.display()
        )))
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    pub fn model_path(&self) -> &PathBuf {
        &self.model_path
    }
}

impl Default for WhisperModel {
    fn default() -> Self {
        Self::new()
    }
} 
