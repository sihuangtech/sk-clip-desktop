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
    load_tts_default_engine_from_models_config().unwrap_or_else(|| "piper".to_string())
}

fn default_tts_engines() -> Vec<TtsEngineOption> {
    load_tts_engine_options_from_models_config().unwrap_or_default()
}

fn find_models_config_path() -> Option<PathBuf> {
    let current_dir = std::env::current_dir().ok()?;
    let candidates = [
        current_dir.join("models").join("models.json"),
        current_dir.parent()?.join("models").join("models.json"),
    ];

    candidates.into_iter().find(|path| path.exists())
}

fn load_models_config_json() -> Option<serde_json::Value> {
    let path = find_models_config_path()?;
    let content = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&content).ok()
}

fn load_tts_default_engine_from_models_config() -> Option<String> {
    let config = load_models_config_json()?;
    config
        .pointer("/models/tts/ui_default_engine")
        .or_else(|| config.pointer("/models/tts/default_engine"))
        .and_then(|value| value.as_str())
        .map(ToString::to_string)
}

fn load_tts_engine_options_from_models_config() -> Option<Vec<TtsEngineOption>> {
    let config = load_models_config_json()?;
    let value = config.pointer("/models/tts/engine_options")?.clone();
    serde_json::from_value(value).ok()
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

        let mut config = if config_path.exists() {
            Self::load_config(&config_path)?
        } else {
            let default_config = AppConfig::default();
            Self::save_config(&config_path, &default_config)?;
            default_config
        };

        Self::refresh_config_driven_options(&mut config);

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
        Self::refresh_config_driven_options(&mut self.config);
        Self::save_config(&self.config_path, &self.config)
    }

    fn refresh_config_driven_options(config: &mut AppConfig) {
        if let Some(tts_engines) = load_tts_engine_options_from_models_config() {
            config.ai.available_tts_engines = tts_engines;
        }

        if !config
            .ai
            .available_tts_engines
            .iter()
            .any(|engine| engine.id == config.ai.selected_tts_engine)
        {
            config.ai.selected_tts_engine = load_tts_default_engine_from_models_config()
                .or_else(|| {
                    config
                        .ai
                        .available_tts_engines
                        .first()
                        .map(|engine| engine.id.clone())
                })
                .unwrap_or_else(|| "piper".to_string());
        }
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
