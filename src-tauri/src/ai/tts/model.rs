// src-tauri/src/ai/tts/model.rs

use std::path::PathBuf;
use log::info;
use crate::models::AppError;

/// TTS模型实现
pub struct TtsModel {
    models_dir: PathBuf,
    initialized: bool,
}

impl TtsModel {
    pub fn new() -> Self {
        Self {
            models_dir: PathBuf::new(),
            initialized: false,
        }
    }

    pub async fn initialize(&mut self) -> Result<(), AppError> {
        info!("正在初始化TTS模型...");
        
        let models_dir = dirs::data_dir()
            .ok_or_else(|| AppError::FileError("无法获取数据目录".to_string()))?
            .join("multisay")
            .join("models")
            .join("tts");
        
        std::fs::create_dir_all(&models_dir).map_err(|e| {
            AppError::FileError(format!("创建TTS模型目录失败: {}", e))
        })?;

        self.models_dir = models_dir;
        
        // 下载TTS模型
        self.download_models().await?;
        
        self.initialized = true;
        info!("TTS模型初始化完成");
        Ok(())
    }

    async fn download_models(&self) -> Result<(), AppError> {
        info!("检查TTS模型目录: {}", self.models_dir.display());
        Err(AppError::ModelInitializationError(
            "真实 TTS 模型下载/加载尚未接入，不能创建占位模型文件。".to_string(),
        ))
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    pub fn models_dir(&self) -> &PathBuf {
        &self.models_dir
    }

    pub fn get_available_voices(&self, language: &str) -> Vec<String> {
        match language {
            "zh" => vec![
                "zh_female_1".to_string(),
                "zh_female_2".to_string(),
                "zh_male_1".to_string(),
                "zh_male_2".to_string(),
            ],
            "en" => vec![
                "en_female_1".to_string(),
                "en_female_2".to_string(),
                "en_male_1".to_string(),
                "en_male_2".to_string(),
            ],
            "ja" => vec![
                "ja_female_1".to_string(),
                "ja_male_1".to_string(),
            ],
            "ko" => vec![
                "ko_female_1".to_string(),
                "ko_male_1".to_string(),
            ],
            _ => vec!["default".to_string()],
        }
    }
}

impl Default for TtsModel {
    fn default() -> Self {
        Self::new()
    }
} 
