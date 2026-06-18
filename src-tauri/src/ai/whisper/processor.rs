// src-tauri/src/ai/whisper/processor.rs

use std::path::PathBuf;
use log::info;
use crate::models::AppError;
use super::{WhisperModel, SpeechRecognitionResult};

/// Whisper语音识别处理器
pub struct WhisperProcessor {
    model: WhisperModel,
}

impl WhisperProcessor {
    pub fn new() -> Self {
        Self {
            model: WhisperModel::new(),
        }
    }

    pub async fn initialize(&mut self) -> Result<(), AppError> {
        self.model.initialize().await
    }

    pub async fn recognize(&self, audio_path: &PathBuf, language: Option<&str>) -> Result<SpeechRecognitionResult, AppError> {
        if !self.model.is_initialized() {
            return Err(AppError::ModelNotInitialized("Whisper模型未初始化".to_string()));
        }

        info!("使用Whisper识别语音: {}", audio_path.display());
        
        // 检查音频文件是否存在
        if !audio_path.exists() {
            return Err(AppError::FileError(format!("音频文件不存在: {}", audio_path.display())));
        }

        let _ = language;
        Err(AppError::AiModelError(
            "真实 Whisper 推理尚未接入，不能返回模拟识别结果。".to_string(),
        ))
    }

    pub fn is_initialized(&self) -> bool {
        self.model.is_initialized()
    }
}

impl Default for WhisperProcessor {
    fn default() -> Self {
        Self::new()
    }
}

/// 便捷函数：直接进行语音识别
pub async fn recognize_speech(
    audio_path: &PathBuf,
    language: Option<&str>,
) -> Result<SpeechRecognitionResult, AppError> {
    let mut processor = WhisperProcessor::new();
    processor.initialize().await?;
    processor.recognize(audio_path, language).await
} 
