pub mod model;
pub mod processor;
pub mod config;

pub use model::TtsModel;
pub use processor::TtsProcessor;
pub use config::TtsConfig;

// 重新导出主要类型
pub use crate::models::AppError;

/// 语音合成结果结构体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SpeechSynthesisResult {
    /// 输入文本
    pub text: String,
    /// 使用的音色
    pub voice: String,
    /// 生成的音频文件路径
    pub output_path: std::path::PathBuf,
    /// 音频时长（秒）
    pub duration: f32,
    /// 采样率
    pub sample_rate: u32,
}

/// 获取可用音色列表
pub fn get_supported_voices(language: &str) -> Vec<String> {
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