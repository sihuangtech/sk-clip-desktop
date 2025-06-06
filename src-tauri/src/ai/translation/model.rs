// src-tauri/src/ai/translation/model.rs

use std::path::PathBuf;
use log::{info, error};
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
        info!("下载翻译模型...");
        
        // 模拟下载多个语言对的模型
        let language_pairs = vec![
            ("zh", "en"), ("en", "zh"),
            ("zh", "ja"), ("ja", "zh"),
            ("zh", "ko"), ("ko", "zh"),
        ];
        
        for (src, tgt) in language_pairs {
            let model_file = self.models_dir.join(format!("{}-{}.bin", src, tgt));
            if !model_file.exists() {
                info!("下载翻译模型: {} -> {}", src, tgt);
                tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                std::fs::write(&model_file, b"TRANSLATION_MODEL_PLACEHOLDER").map_err(|e| {
                    AppError::FileError(format!("保存翻译模型失败: {}", e))
                })?;
            }
        }
        
        info!("翻译模型下载完成");
        Ok(())
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