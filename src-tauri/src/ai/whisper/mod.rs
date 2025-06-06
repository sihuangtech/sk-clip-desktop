pub mod model;
pub mod processor;

pub use model::WhisperModel;
pub use processor::WhisperProcessor;

// 重新导出主要类型
pub use crate::models::AppError;

/// Whisper语音识别结果
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SpeechRecognitionResult {
    /// 识别出的文本内容
    pub text: String,
    /// 识别置信度 (0.0 - 1.0)
    pub confidence: f32,
    /// 语言代码
    pub language: String,
    /// 时间戳信息 (开始时间, 结束时间) - 单位：秒
    pub timestamps: Vec<(f32, f32, String)>,
}

/// 支持的语音识别语言列表
pub fn get_supported_languages() -> Vec<&'static str> {
    vec!["zh", "en", "ja", "ko", "es", "fr", "de", "ru", "pt", "it"]
} 