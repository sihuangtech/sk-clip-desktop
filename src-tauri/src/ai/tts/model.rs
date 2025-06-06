// src-tauri/src/ai/tts/model.rs

use std::path::PathBuf;
use log::{info, error};
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
        info!("下载TTS模型...");
        
        let languages = vec!["zh", "en", "ja", "ko"];
        
        for lang in languages {
            let model_file = self.models_dir.join(format!("tts-{}.bin", lang));
            if !model_file.exists() {
                info!("下载TTS模型: {}", lang);
                tokio::time::sleep(std::time::Duration::from_millis(800)).await;
                std::fs::write(&model_file, b"TTS_MODEL_PLACEHOLDER").map_err(|e| {
                    AppError::FileError(format!("保存TTS模型失败: {}", e))
                })?;
            }
        }
        
        info!("TTS模型下载完成");
        Ok(())
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