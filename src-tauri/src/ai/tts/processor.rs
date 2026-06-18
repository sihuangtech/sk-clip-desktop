use std::path::PathBuf;
use crate::models::AppError;
use log::info;
use super::{TtsModel, SpeechSynthesisResult};

/// TTS处理器
pub struct TtsProcessor {
    model: TtsModel,
}

impl TtsProcessor {
    pub fn new() -> Self {
        Self {
            model: TtsModel::new(),
        }
    }

    pub async fn initialize(&mut self) -> Result<(), AppError> {
        self.model.initialize().await
    }

    pub async fn synthesize(&self, text: &str, voice: Option<&str>, output_path: &PathBuf) -> Result<SpeechSynthesisResult, AppError> {
        if !self.model.is_initialized() {
            return Err(AppError::ModelNotInitialized("TTS模型未初始化".to_string()));
        }

        info!("合成语音: {} -> {}", text, output_path.display());
        
        let _ = (voice, output_path);
        Err(AppError::AiModelError(
            "真实 TTS 合成尚未接入，不能创建模拟音频文件。".to_string(),
        ))
    }

    pub fn is_initialized(&self) -> bool {
        self.model.is_initialized()
    }
}

impl Default for TtsProcessor {
    fn default() -> Self {
        Self::new()
    }
} 
