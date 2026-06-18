// src-tauri/src/ai/translation/model.rs

use std::path::PathBuf;
use log::info;
use crate::models::AppError;

/// 翻译模型实现
pub struct TranslationModel {
    models_dir: PathBuf,
    initialized: bool,
}

impl TranslationModel {
    pub fn new() -> Self {
        Self {
            models_dir: PathBuf::new(),
            initialized: false,
        }
    }

    pub async fn initialize(&mut self) -> Result<(), AppError> {
        info!("正在初始化翻译模型...");
        
        let models_dir = dirs::data_dir()
            .ok_or_else(|| AppError::FileError("无法获取数据目录".to_string()))?
            .join("multisay")
            .join("models")
            .join("translation");
        
        std::fs::create_dir_all(&models_dir).map_err(|e| {
            AppError::FileError(format!("创建翻译模型目录失败: {}", e))
        })?;

        self.models_dir = models_dir;
        
        // 下载必要的翻译模型
        self.download_models().await?;
        
        self.initialized = true;
        info!("翻译模型初始化完成");
        Ok(())
    }

    async fn download_models(&self) -> Result<(), AppError> {
        info!("检查翻译模型目录: {}", self.models_dir.display());
        Err(AppError::ModelInitializationError(
            "真实翻译模型下载/加载尚未接入，不能创建占位模型文件。".to_string(),
        ))
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    pub fn is_translation_pair_supported(&self, source_lang: &str, target_lang: &str) -> bool {
        let supported_languages = vec!["zh", "en", "ja", "ko", "es", "fr", "de", "ru", "pt"];
        supported_languages.contains(&source_lang) && supported_languages.contains(&target_lang)
    }

    pub fn models_dir(&self) -> &PathBuf {
        &self.models_dir
    }
}

impl Default for TranslationModel {
    fn default() -> Self {
        Self::new()
    }
} 
