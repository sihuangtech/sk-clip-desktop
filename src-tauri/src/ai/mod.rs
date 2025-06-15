pub mod whisper;
pub mod translation;
pub mod tts;

// 重新导出主要类型和功能
pub use whisper::{WhisperProcessor, SpeechRecognitionResult, get_supported_languages as get_whisper_languages};
pub use translation::{TranslationProcessor, TranslationResult, get_supported_language_pairs};
pub use tts::{TtsProcessor, SpeechSynthesisResult, get_supported_voices};

use crate::models::AppError;
use crate::config::AppConfig;

/// AI服务管理器
pub struct AiServiceManager {
    whisper: Option<WhisperProcessor>,
    translation: Option<TranslationProcessor>,
    tts: Option<TtsProcessor>,
    _config: AppConfig,
}

impl AiServiceManager {
    pub fn new(config: AppConfig) -> Self {
        Self {
            whisper: None,
            translation: None,
            tts: None,
            _config: config,
        }
    }

    /// 初始化所有AI服务
    pub async fn initialize_all(&mut self) -> Result<(), AppError> {
        self.initialize_whisper().await?;
        self.initialize_translation().await?;
        self.initialize_tts().await?;
        Ok(())
    }

    /// 初始化Whisper服务
    pub async fn initialize_whisper(&mut self) -> Result<(), AppError> {
        let mut processor = WhisperProcessor::new();
        processor.initialize().await?;
        self.whisper = Some(processor);
        Ok(())
    }

    /// 初始化翻译服务
    pub async fn initialize_translation(&mut self) -> Result<(), AppError> {
        let mut processor = TranslationProcessor::new();
        processor.initialize().await?;
        self.translation = Some(processor);
        Ok(())
    }

    /// 初始化TTS服务
    pub async fn initialize_tts(&mut self) -> Result<(), AppError> {
        let mut processor = TtsProcessor::new();
        processor.initialize().await?;
        self.tts = Some(processor);
        Ok(())
    }

    /// 获取Whisper处理器
    pub fn get_whisper(&self) -> Option<&WhisperProcessor> {
        self.whisper.as_ref()
    }

    /// 获取翻译处理器
    pub fn get_translation(&self) -> Option<&TranslationProcessor> {
        self.translation.as_ref()
    }

    /// 获取TTS处理器
    pub fn get_tts(&self) -> Option<&TtsProcessor> {
        self.tts.as_ref()
    }

    /// 检查所有服务状态
    pub fn get_service_status(&self) -> AiServiceStatus {
        AiServiceStatus {
            whisper_initialized: self.whisper.as_ref().map_or(false, |w| w.is_initialized()),
            translation_initialized: self.translation.as_ref().map_or(false, |t| t.is_initialized()),
            tts_initialized: self.tts.as_ref().map_or(false, |t| t.is_initialized()),
        }
    }
}

/// AI服务状态
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AiServiceStatus {
    pub whisper_initialized: bool,
    pub translation_initialized: bool,
    pub tts_initialized: bool,
}

impl AiServiceStatus {
    pub fn all_initialized(&self) -> bool {
        self.whisper_initialized && self.translation_initialized && self.tts_initialized
    }
} 