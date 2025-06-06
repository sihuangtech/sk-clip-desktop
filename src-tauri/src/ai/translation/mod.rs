pub mod model;
pub mod processor;

pub use model::TranslationModel;
pub use processor::TranslationProcessor;

// 重新导出主要类型
pub use crate::models::AppError;

/// 翻译结果结构体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TranslationResult {
    /// 原始文本
    pub source_text: String,
    /// 翻译后的文本
    pub translated_text: String,
    /// 源语言代码
    pub source_language: String,
    /// 目标语言代码
    pub target_language: String,
    /// 翻译置信度 (0.0 - 1.0)
    pub confidence: f32,
}

/// 获取支持的翻译语言对
pub fn get_supported_language_pairs() -> Vec<(String, String)> {
    let languages = vec!["zh", "en", "ja", "ko", "es", "fr", "de", "ru", "pt"];
    let mut pairs = Vec::new();
    
    for source in &languages {
        for target in &languages {
            if source != target {
                pairs.push((source.to_string(), target.to_string()));
            }
        }
    }
    
    pairs
} 