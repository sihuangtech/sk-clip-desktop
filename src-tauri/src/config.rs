use crate::models::AppError;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// 应用配置结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// AI模型配置
    pub ai: AiConfig,
    /// 视频处理配置
    pub video: VideoConfig,
    /// 文档处理配置
    pub document: DocumentConfig,
    /// 用户界面配置
    pub ui: UiConfig,
}

/// AI模型配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    /// Whisper模型路径
    pub whisper_model_path: Option<PathBuf>,
    /// TTS模型路径
    pub tts_model_path: Option<PathBuf>,
    /// 当前选择的TTS引擎
    #[serde(default = "default_tts_engine")]
    pub selected_tts_engine: String,
    /// 可选TTS引擎列表
    #[serde(default = "default_tts_engines")]
    pub available_tts_engines: Vec<TtsEngineOption>,
    /// 翻译模型路径
    pub translation_model_path: Option<PathBuf>,
    /// 默认语言
    pub default_language: String,
    /// 支持的语言列表
    pub supported_languages: Vec<String>,
}

/// 可选TTS引擎元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TtsEngineOption {
    /// 引擎ID，用于配置保存和命令调用
    pub id: String,
    /// 展示名称
    pub name: String,
    /// 引擎类别
    pub category: String,
    /// 本地部署体积/资源占用描述
    pub footprint: String,
    /// 开源许可证或权重许可证提示
    pub license: String,
    /// 代码仓库地址
    pub repository_url: String,
    /// 适合的使用场景
    pub recommended_use: String,
}

/// 视频处理配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoConfig {
    /// 默认输出格式
    pub default_output_format: String,
    /// 默认视频质量
    pub default_quality: String,
    /// 最大视频文件大小 (MB)
    pub max_file_size_mb: u64,
    /// 临时文件目录
    pub temp_dir: Option<PathBuf>,
}

/// 文档处理配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentConfig {
    /// 支持的文档格式
    pub supported_formats: Vec<String>,
    /// 最大文档大小 (MB)
    pub max_file_size_mb: u64,
    /// PDF DPI设置
    pub pdf_dpi: u32,
}

/// 用户界面配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    /// 主题
    pub theme: String,
    /// 语言
    pub language: String,
    /// 窗口大小
    pub window_size: (u32, u32),
    /// 是否记住窗口位置
    pub remember_window_position: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            ai: AiConfig {
                whisper_model_path: None,
                tts_model_path: None,
                selected_tts_engine: default_tts_engine(),
                available_tts_engines: default_tts_engines(),
                translation_model_path: None,
                default_language: "zh".to_string(),
                supported_languages: vec![
                    "zh".to_string(),
                    "en".to_string(),
                    "ja".to_string(),
                    "ko".to_string(),
                    "es".to_string(),
                    "fr".to_string(),
                    "de".to_string(),
                    "ru".to_string(),
                ],
            },
            video: VideoConfig {
                default_output_format: "mp4".to_string(),
                default_quality: "high".to_string(),
                max_file_size_mb: 1024,
                temp_dir: None,
            },
            document: DocumentConfig {
                supported_formats: vec![
                    "pdf".to_string(),
                    "pptx".to_string(),
                    "md".to_string(),
                    "docx".to_string(),
                ],
                max_file_size_mb: 100,
                pdf_dpi: 300,
            },
            ui: UiConfig {
                theme: "light".to_string(),
                language: "zh".to_string(),
                window_size: (1200, 800),
                remember_window_position: true,
            },
        }
    }
}

fn default_tts_engine() -> String {
    "piper".to_string()
}

fn default_tts_engines() -> Vec<TtsEngineOption> {
    vec![
        TtsEngineOption {
            id: "espeak-ng".to_string(),
            name: "eSpeak NG".to_string(),
            category: "极轻量规则引擎".to_string(),
            footprint: "几 MB，CPU 运行".to_string(),
            license: "GPL-3.0".to_string(),
            repository_url: "https://github.com/espeak-ng/espeak-ng".to_string(),
            recommended_use: "兜底朗读、调试、多语言覆盖".to_string(),
        },
        TtsEngineOption {
            id: "flite".to_string(),
            name: "Flite".to_string(),
            category: "极轻量嵌入式引擎".to_string(),
            footprint: "小体积，CPU 运行".to_string(),
            license: "BSD-like".to_string(),
            repository_url: "https://github.com/festvox/flite".to_string(),
            recommended_use: "英文兜底朗读、低资源设备".to_string(),
        },
        TtsEngineOption {
            id: "tinytts".to_string(),
            name: "TinyTTS".to_string(),
            category: "极轻量神经网络".to_string(),
            footprint: "约 3.4 MB ONNX，CPU 运行".to_string(),
            license: "MIT".to_string(),
            repository_url: "https://github.com/tronghieuit/tiny-tts".to_string(),
            recommended_use: "英文轻量实验、极小安装包".to_string(),
        },
        TtsEngineOption {
            id: "kitten-tts".to_string(),
            name: "KittenTTS".to_string(),
            category: "轻量神经网络".to_string(),
            footprint: "约 25-80 MB ONNX，CPU 运行".to_string(),
            license: "Apache-2.0".to_string(),
            repository_url: "https://github.com/KittenML/KittenTTS".to_string(),
            recommended_use: "轻量自然朗读、桌面端默认候选".to_string(),
        },
        TtsEngineOption {
            id: "piper".to_string(),
            name: "Piper".to_string(),
            category: "轻量神经网络".to_string(),
            footprint: "按语音包下载，CPU 运行".to_string(),
            license: "MIT / GPL 分支需分别确认".to_string(),
            repository_url: "https://github.com/rhasspy/piper".to_string(),
            recommended_use: "稳定离线朗读、默认内置引擎".to_string(),
        },
        TtsEngineOption {
            id: "kokoro".to_string(),
            name: "Kokoro ONNX".to_string(),
            category: "中轻量高质量神经网络".to_string(),
            footprint: "量化约 80 MB，CPU 可运行".to_string(),
            license: "Apache-2.0".to_string(),
            repository_url: "https://github.com/thewh1teagle/kokoro-onnx".to_string(),
            recommended_use: "更自然的讲解配音、高质量默认候选".to_string(),
        },
        TtsEngineOption {
            id: "sherpa-onnx".to_string(),
            name: "sherpa-onnx TTS".to_string(),
            category: "跨平台 ONNX 运行层".to_string(),
            footprint: "取决于所选模型，支持 CPU".to_string(),
            license: "Apache-2.0".to_string(),
            repository_url: "https://github.com/k2-fsa/sherpa-onnx".to_string(),
            recommended_use: "统一接入 Piper、Kokoro、VITS 等本地模型".to_string(),
        },
        TtsEngineOption {
            id: "gpt-sovits".to_string(),
            name: "GPT-SoVITS".to_string(),
            category: "声音克隆/高级配音".to_string(),
            footprint: "较重，建议 Python sidecar/GPU 可选".to_string(),
            license: "MIT".to_string(),
            repository_url: "https://github.com/RVC-Boss/GPT-SoVITS".to_string(),
            recommended_use: "视频翻译配音、少样本音色克隆".to_string(),
        },
        TtsEngineOption {
            id: "f5-tts".to_string(),
            name: "F5-TTS".to_string(),
            category: "声音克隆/高级配音".to_string(),
            footprint: "较重，建议 Python sidecar/GPU 可选".to_string(),
            license: "代码 MIT，预训练权重需单独确认".to_string(),
            repository_url: "https://github.com/SWivid/F5-TTS".to_string(),
            recommended_use: "高质量克隆实验、非默认高级引擎".to_string(),
        },
    ]
}

/// 配置管理器
pub struct ConfigManager {
    config: AppConfig,
    config_path: PathBuf,
}

impl ConfigManager {
    pub fn new() -> Result<Self, AppError> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| AppError::ConfigError("无法获取配置目录".to_string()))?
            .join("sk-clip");

        std::fs::create_dir_all(&config_dir)
            .map_err(|e| AppError::ConfigError(format!("创建配置目录失败: {}", e)))?;

        let config_path = config_dir.join("config.json");

        let config = if config_path.exists() {
            Self::load_config(&config_path)?
        } else {
            let default_config = AppConfig::default();
            Self::save_config(&config_path, &default_config)?;
            default_config
        };

        Ok(Self {
            config,
            config_path,
        })
    }

    pub fn get_config(&self) -> &AppConfig {
        &self.config
    }

    pub fn update_config(&mut self, config: AppConfig) -> Result<(), AppError> {
        self.config = config;
        Self::save_config(&self.config_path, &self.config)
    }

    fn load_config(path: &PathBuf) -> Result<AppConfig, AppError> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| AppError::ConfigError(format!("读取配置文件失败: {}", e)))?;

        serde_json::from_str(&content)
            .map_err(|e| AppError::ConfigError(format!("解析配置文件失败: {}", e)))
    }

    fn save_config(path: &PathBuf, config: &AppConfig) -> Result<(), AppError> {
        let content = serde_json::to_string_pretty(config)
            .map_err(|e| AppError::ConfigError(format!("序列化配置失败: {}", e)))?;

        std::fs::write(path, content)
            .map_err(|e| AppError::ConfigError(format!("保存配置文件失败: {}", e)))
    }
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self::new().expect("创建配置管理器失败")
    }
}
