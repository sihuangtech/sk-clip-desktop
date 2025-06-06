// src-tauri/src/model_manager/config.rs

// 模型配置管理器，负责读取和解析模型配置文件

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use log::{info, warn, error};
use crate::models::AppError;

/// 模型配置结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub version: String,
    pub models: ModelsConfig,
    pub download_settings: DownloadSettings,
    pub storage_settings: StorageSettings,
}

/// 所有模型的配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelsConfig {
    pub whisper: WhisperConfig,
    pub translation: TranslationConfig,
    pub tts: TtsConfig,
}

/// Whisper模型配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhisperConfig {
    pub enabled: bool,
    pub default_model: String,
    pub models: HashMap<String, WhisperModelInfo>,
}

/// Whisper模型信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhisperModelInfo {
    pub size_mb: u64,
    pub download_url: String,
    pub description: String,
    pub languages: Vec<String>,
}

/// 翻译模型配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationConfig {
    pub enabled: bool,
    pub default_provider: String,
    pub providers: HashMap<String, TranslationProvider>,
}

/// 翻译提供商
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationProvider {
    pub description: String,
    pub base_url: String,
    pub models: HashMap<String, TranslationModelInfo>,
}

/// 翻译模型信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationModelInfo {
    pub model_name: String,
    pub size_mb: u64,
    #[serde(flatten)]
    pub model_type: TranslationModelType,
}

/// 翻译模型类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TranslationModelType {
    Bilateral {
        source_lang: String,
        target_lang: String,
    },
    Multilingual {
        languages: Vec<String>,
    },
}

/// TTS模型配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TtsConfig {
    pub enabled: bool,
    pub default_engine: String,
    pub engines: HashMap<String, TtsEngine>,
}

/// TTS引擎
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TtsEngine {
    pub description: String,
    #[serde(flatten)]
    pub engine_type: TtsEngineType,
}

/// TTS引擎类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TtsEngineType {
    Simple {
        size_mb: u64,
        voices: HashMap<String, Vec<String>>,
    },
    Neural {
        base_url: String,
        models: HashMap<String, TtsModelInfo>,
    },
}

/// TTS模型信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TtsModelInfo {
    pub model_name: String,
    pub voice_name: String,
    pub size_mb: u64,
    pub gender: String,
    pub language: String,
}

/// 下载设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadSettings {
    pub max_concurrent_downloads: u32,
    pub retry_attempts: u32,
    pub timeout_seconds: u64,
    pub verify_checksum: bool,
    pub auto_download_on_startup: bool,
    pub preferred_mirror: String,
}

/// 存储设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageSettings {
    pub max_cache_size_gb: u64,
    pub auto_cleanup_old_models: bool,
    pub compress_models: bool,
}

/// 用户配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserConfig {
    pub user_preferences: UserPreferences,
    pub download_preferences: DownloadPreferences,
    pub storage_preferences: StoragePreferences,
    pub advanced_settings: AdvancedSettings,
}

/// 用户偏好设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub whisper: WhisperUserPrefs,
    pub translation: TranslationUserPrefs,
    pub tts: TtsUserPrefs,
}

/// Whisper用户偏好
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhisperUserPrefs {
    pub preferred_model: String,
    pub auto_download: bool,
    pub language_priority: Vec<String>,
}

/// 翻译用户偏好
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationUserPrefs {
    pub preferred_provider: String,
    pub auto_download: bool,
    pub cache_translations: bool,
}

/// TTS用户偏好
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TtsUserPrefs {
    pub preferred_engine: String,
    pub preferred_voice: HashMap<String, String>,
    pub speech_rate: f32,
    pub volume: f32,
}

/// 下载偏好
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadPreferences {
    pub wifi_only: bool,
    pub background_download: bool,
    pub notification_on_complete: bool,
    pub max_download_speed_mbps: u64,
}

/// 存储偏好
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoragePreferences {
    pub custom_model_path: String,
    pub auto_cleanup_enabled: bool,
    pub max_storage_gb: u64,
    pub backup_models: bool,
}

/// 高级设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedSettings {
    pub gpu_acceleration: String,
    pub cpu_threads: u32,
    pub memory_limit_gb: u64,
    pub debug_mode: bool,
}

/// 模型配置管理器
pub struct ModelConfigManager {
    model_config: ModelConfig,
    user_config: UserConfig,
    models_dir: PathBuf,
}

impl ModelConfigManager {
    /// 创建新的模型配置管理器
    pub fn new() -> Result<Self, AppError> {
        let models_dir = Self::get_models_directory()?;
        
        let model_config = Self::load_model_config(&models_dir)?;
        let user_config = Self::load_user_config(&models_dir)?;
        
        Ok(Self {
            model_config,
            user_config,
            models_dir,
        })
    }
    
    /// 获取模型目录路径
    fn get_models_directory() -> Result<PathBuf, AppError> {
        let current_dir = std::env::current_dir()
            .map_err(|e| AppError::ConfigError(format!("获取当前目录失败: {}", e)))?;
        
        // 查找models目录
        let models_dir = current_dir.join("models");
        if models_dir.exists() {
            return Ok(models_dir);
        }
        
        // 如果在src-tauri目录中，向上查找
        let parent_models_dir = current_dir.parent()
            .ok_or_else(|| AppError::ConfigError("无法找到父目录".to_string()))?
            .join("models");
        
        if parent_models_dir.exists() {
            return Ok(parent_models_dir);
        }
        
        Err(AppError::ConfigError("无法找到models目录".to_string()))
    }
    
    /// 加载模型配置
    fn load_model_config(models_dir: &PathBuf) -> Result<ModelConfig, AppError> {
        let config_path = models_dir.join("models.json");
        
        if !config_path.exists() {
            error!("模型配置文件不存在: {}", config_path.display());
            return Err(AppError::ConfigError("模型配置文件不存在".to_string()));
        }
        
        let content = std::fs::read_to_string(&config_path)
            .map_err(|e| AppError::ConfigError(format!("读取模型配置文件失败: {}", e)))?;
        
        let config: ModelConfig = serde_json::from_str(&content)
            .map_err(|e| AppError::ConfigError(format!("解析模型配置文件失败: {}", e)))?;
        
        info!("成功加载模型配置，版本: {}", config.version);
        Ok(config)
    }
    
    /// 加载用户配置
    fn load_user_config(models_dir: &PathBuf) -> Result<UserConfig, AppError> {
        let config_path = models_dir.join("user_config.json");
        
        if !config_path.exists() {
            warn!("用户配置文件不存在，使用默认配置");
            return Ok(Self::default_user_config());
        }
        
        let content = std::fs::read_to_string(&config_path)
            .map_err(|e| AppError::ConfigError(format!("读取用户配置文件失败: {}", e)))?;
        
        let config: UserConfig = serde_json::from_str(&content)
            .map_err(|e| AppError::ConfigError(format!("解析用户配置文件失败: {}", e)))?;
        
        info!("成功加载用户配置");
        Ok(config)
    }
    
    /// 默认用户配置
    fn default_user_config() -> UserConfig {
        UserConfig {
            user_preferences: UserPreferences {
                whisper: WhisperUserPrefs {
                    preferred_model: "base".to_string(),
                    auto_download: true,
                    language_priority: vec!["zh".to_string(), "en".to_string()],
                },
                translation: TranslationUserPrefs {
                    preferred_provider: "opus_mt".to_string(),
                    auto_download: true,
                    cache_translations: true,
                },
                tts: TtsUserPrefs {
                    preferred_engine: "neural".to_string(),
                    preferred_voice: {
                        let mut voices = HashMap::new();
                        voices.insert("zh".to_string(), "zh_female".to_string());
                        voices.insert("en".to_string(), "en_female".to_string());
                        voices
                    },
                    speech_rate: 1.0,
                    volume: 0.8,
                },
            },
            download_preferences: DownloadPreferences {
                wifi_only: false,
                background_download: true,
                notification_on_complete: true,
                max_download_speed_mbps: 0,
            },
            storage_preferences: StoragePreferences {
                custom_model_path: String::new(),
                auto_cleanup_enabled: true,
                max_storage_gb: 5,
                backup_models: false,
            },
            advanced_settings: AdvancedSettings {
                gpu_acceleration: "auto".to_string(),
                cpu_threads: 0,
                memory_limit_gb: 4,
                debug_mode: false,
            },
        }
    }
    
    /// 获取模型配置
    pub fn get_model_config(&self) -> &ModelConfig {
        &self.model_config
    }
    
    /// 获取用户配置
    pub fn get_user_config(&self) -> &UserConfig {
        &self.user_config
    }
    
    /// 获取模型目录
    pub fn get_models_dir(&self) -> &PathBuf {
        &self.models_dir
    }
    
    /// 保存用户配置
    pub fn save_user_config(&self) -> Result<(), AppError> {
        let config_path = self.models_dir.join("user_config.json");
        let content = serde_json::to_string_pretty(&self.user_config)
            .map_err(|e| AppError::ConfigError(format!("序列化用户配置失败: {}", e)))?;
        
        std::fs::write(&config_path, content)
            .map_err(|e| AppError::ConfigError(format!("保存用户配置失败: {}", e)))?;
        
        info!("用户配置已保存到: {}", config_path.display());
        Ok(())
    }
} 