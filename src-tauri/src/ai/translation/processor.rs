// src-tauri/src/ai/translation/processor.rs

use log::info;
use crate::models::AppError;
use super::{TranslationModel, TranslationResult};

/// 翻译处理器
pub struct TranslationProcessor {
    model: TranslationModel,
}

impl TranslationProcessor {
    pub fn new() -> Self {
        Self {
            model: TranslationModel::new(),
        }
    }

    pub async fn initialize(&mut self) -> Result<(), AppError> {
        self.model.initialize().await
    }

    pub async fn translate(&self, text: &str, source_lang: &str, target_lang: &str) -> Result<TranslationResult, AppError> {
        if !self.model.is_initialized() {
            return Err(AppError::ModelNotInitialized("翻译模型未初始化".to_string()));
        }

        info!("翻译文本: {} -> {}, 长度: {}", source_lang, target_lang, text.len());
        
        // 检查语言对是否支持
        if !self.model.is_translation_pair_supported(source_lang, target_lang) {
            return Err(AppError::UnsupportedLanguage(
                format!("不支持的翻译语言对: {} -> {}", source_lang, target_lang)
            ));
        }
        
        Err(AppError::AiModelError(
            "真实翻译推理尚未接入，不能返回模拟翻译结果。".to_string(),
        ))
    }

    pub fn is_initialized(&self) -> bool {
        self.model.is_initialized()
    }
}

impl Default for TranslationProcessor {
    fn default() -> Self {
        Self::new()
    }
}

/// 便捷函数：直接进行文本翻译
pub async fn translate_text(
    text: &str,
    source_lang: &str,
    target_lang: &str,
) -> Result<TranslationResult, AppError> {
    let mut processor = TranslationProcessor::new();
    processor.initialize().await?;
    processor.translate(text, source_lang, target_lang).await
} 
