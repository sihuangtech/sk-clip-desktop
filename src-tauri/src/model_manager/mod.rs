// src-tauri/src/model_manager/mod.rs

// 模型管理器模块，负责AI模型的配置、下载和管理

pub mod config;
pub mod downloader;
pub mod storage;
pub mod proxy;

pub use config::{ModelConfigManager, WhisperModelInfo, ModelConfig};
pub use downloader::ModelDownloader;
pub use storage::ModelStorageManager;
pub use proxy::{ProxyManager, ProxyInfo, ProxyType, ProxyProfile}; 