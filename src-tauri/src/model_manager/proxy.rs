// src-tauri/src/model_manager/proxy.rs

// 代理管理器，负责处理网络代理配置和连接

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use log::{info, warn, error, debug};
use crate::models::AppError;

/// 代理配置结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyConfig {
    pub proxy_settings: ProxySettings,
    pub proxy_profiles: HashMap<String, ProxyProfile>,
    pub download_settings: DownloadSettings,
    pub mirror_settings: MirrorSettings,
    pub security_settings: SecuritySettings,
}

/// 代理设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxySettings {
    pub enabled: bool,
    #[serde(rename = "type")]
    pub proxy_type: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub bypass_list: Vec<String>,
}

/// 代理配置文件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyProfile {
    pub name: String,
    #[serde(rename = "type")]
    pub proxy_type: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub description: String,
}

/// 下载设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadSettings {
    pub auto_detect_proxy: bool,
    pub fallback_to_direct: bool,
    pub proxy_timeout_seconds: u64,
    pub test_urls: Vec<String>,
    pub retry_with_different_proxy: bool,
    pub max_proxy_retries: u32,
}

/// 镜像设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MirrorSettings {
    pub use_mirrors_when_proxy_fails: bool,
    pub preferred_mirrors: HashMap<String, Vec<String>>,
}

/// 安全设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecuritySettings {
    pub verify_ssl: bool,
    pub allow_insecure_connections: bool,
    pub custom_ca_bundle: String,
    pub user_agent: String,
}

/// 代理类型枚举
#[derive(Debug, Clone, PartialEq)]
pub enum ProxyType {
    Http,
    Https,
    Socks4,
    Socks5,
    Direct,
}

impl From<&str> for ProxyType {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "http" => ProxyType::Http,
            "https" => ProxyType::Https,
            "socks4" => ProxyType::Socks4,
            "socks5" => ProxyType::Socks5,
            "direct" => ProxyType::Direct,
            _ => ProxyType::Http,
        }
    }
}

impl ToString for ProxyType {
    fn to_string(&self) -> String {
        match self {
            ProxyType::Http => "http".to_string(),
            ProxyType::Https => "https".to_string(),
            ProxyType::Socks4 => "socks4".to_string(),
            ProxyType::Socks5 => "socks5".to_string(),
            ProxyType::Direct => "direct".to_string(),
        }
    }
}

/// 代理管理器
pub struct ProxyManager {
    config: ProxyConfig,
    models_dir: PathBuf,
}

impl ProxyManager {
    /// 创建新的代理管理器
    pub fn new(models_dir: PathBuf) -> Result<Self, AppError> {
        let config = Self::load_proxy_config(&models_dir)?;
        
        Ok(Self {
            config,
            models_dir,
        })
    }
    
    /// 加载代理配置
    fn load_proxy_config(models_dir: &PathBuf) -> Result<ProxyConfig, AppError> {
        let config_path = models_dir.join("proxy_config.json");
        
        if !config_path.exists() {
            warn!("代理配置文件不存在，使用默认配置");
            return Ok(Self::default_proxy_config());
        }
        
        let content = std::fs::read_to_string(&config_path)
            .map_err(|e| AppError::ConfigError(format!("读取代理配置文件失败: {}", e)))?;
        
        let config: ProxyConfig = serde_json::from_str(&content)
            .map_err(|e| AppError::ConfigError(format!("解析代理配置文件失败: {}", e)))?;
        
        info!("成功加载代理配置");
        Ok(config)
    }
    
    /// 默认代理配置
    fn default_proxy_config() -> ProxyConfig {
        ProxyConfig {
            proxy_settings: ProxySettings {
                enabled: false,
                proxy_type: "http".to_string(),
                host: String::new(),
                port: 0,
                username: String::new(),
                password: String::new(),
                bypass_list: vec![
                    "localhost".to_string(),
                    "127.0.0.1".to_string(),
                    "*.local".to_string(),
                ],
            },
            proxy_profiles: HashMap::new(),
            download_settings: DownloadSettings {
                auto_detect_proxy: true,
                fallback_to_direct: true,
                proxy_timeout_seconds: 30,
                test_urls: vec![
                    "https://www.google.com".to_string(),
                    "https://huggingface.co".to_string(),
                    "https://github.com".to_string(),
                ],
                retry_with_different_proxy: true,
                max_proxy_retries: 3,
            },
            mirror_settings: MirrorSettings {
                use_mirrors_when_proxy_fails: true,
                preferred_mirrors: HashMap::new(),
            },
            security_settings: SecuritySettings {
                verify_ssl: true,
                allow_insecure_connections: false,
                custom_ca_bundle: String::new(),
                user_agent: "MultisayApp/1.0.0".to_string(),
            },
        }
    }
    
    /// 获取当前代理配置
    pub fn get_current_proxy(&self) -> Option<ProxyInfo> {
        if !self.config.proxy_settings.enabled {
            return None;
        }
        
        Some(ProxyInfo {
            proxy_type: ProxyType::from(self.config.proxy_settings.proxy_type.as_str()),
            host: self.config.proxy_settings.host.clone(),
            port: self.config.proxy_settings.port,
            username: if self.config.proxy_settings.username.is_empty() {
                None
            } else {
                Some(self.config.proxy_settings.username.clone())
            },
            password: if self.config.proxy_settings.password.is_empty() {
                None
            } else {
                Some(self.config.proxy_settings.password.clone())
            },
        })
    }
    
    /// 测试代理连接
    pub async fn test_proxy(&self, proxy_info: &ProxyInfo) -> Result<bool, AppError> {
        info!("测试代理连接: {}:{}", proxy_info.host, proxy_info.port);
        
        // TODO: 实现实际的代理测试逻辑
        // 这里只是模拟测试过程
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        
        // 模拟测试结果
        let success = !proxy_info.host.is_empty() && proxy_info.port > 0;
        
        if success {
            info!("代理连接测试成功");
        } else {
            warn!("代理连接测试失败");
        }
        
        Ok(success)
    }
    
    /// 自动检测系统代理
    pub fn auto_detect_proxy(&self) -> Option<ProxyInfo> {
        if !self.config.download_settings.auto_detect_proxy {
            return None;
        }
        
        info!("自动检测系统代理设置");
        
        // TODO: 实现实际的系统代理检测逻辑
        // 这里只是模拟检测过程
        
        // 检查常见的代理端口
        let common_proxies = vec![
            ("127.0.0.1", 7890, ProxyType::Http),  // Clash
            ("127.0.0.1", 1080, ProxyType::Socks5), // V2Ray/Shadowsocks
            ("127.0.0.1", 8080, ProxyType::Http),  // 通用HTTP代理
        ];
        
        for (host, port, proxy_type) in common_proxies {
            debug!("检测代理: {}:{}", host, port);
            // 这里应该实际测试连接
            // 暂时返回第一个找到的代理配置
            if self.is_port_open(host, port) {
                info!("检测到代理: {}:{}", host, port);
                return Some(ProxyInfo {
                    proxy_type,
                    host: host.to_string(),
                    port,
                    username: None,
                    password: None,
                });
            }
        }
        
        None
    }
    
    /// 检查端口是否开放（简化版本）
    fn is_port_open(&self, _host: &str, _port: u16) -> bool {
        // TODO: 实现实际的端口检测逻辑
        // 这里只是模拟
        false
    }
    
    /// 获取镜像URL
    pub fn get_mirror_url(&self, original_url: &str) -> String {
        if !self.config.mirror_settings.use_mirrors_when_proxy_fails {
            return original_url.to_string();
        }
        
        // 检查是否有对应的镜像配置
        for (service, mirrors) in &self.config.mirror_settings.preferred_mirrors {
            if original_url.contains(service) {
                if let Some(mirror) = mirrors.first() {
                    let mirrored_url = original_url.replace(
                        &format!("https://{}", service), 
                        mirror
                    );
                    info!("使用镜像URL: {} -> {}", original_url, mirrored_url);
                    return mirrored_url;
                }
            }
        }
        
        original_url.to_string()
    }
    
    /// 获取所有可用的代理配置文件
    pub fn get_proxy_profiles(&self) -> &HashMap<String, ProxyProfile> {
        &self.config.proxy_profiles
    }
    
    /// 应用代理配置文件
    pub fn apply_proxy_profile(&mut self, profile_name: &str) -> Result<(), AppError> {
        let profile = self.config.proxy_profiles.get(profile_name)
            .ok_or_else(|| AppError::ConfigError(format!("代理配置文件不存在: {}", profile_name)))?;
        
        self.config.proxy_settings.enabled = true;
        self.config.proxy_settings.proxy_type = profile.proxy_type.clone();
        self.config.proxy_settings.host = profile.host.clone();
        self.config.proxy_settings.port = profile.port;
        self.config.proxy_settings.username = profile.username.clone();
        self.config.proxy_settings.password = profile.password.clone();
        
        info!("应用代理配置文件: {}", profile_name);
        self.save_config()
    }
    
    /// 禁用代理
    pub fn disable_proxy(&mut self) -> Result<(), AppError> {
        self.config.proxy_settings.enabled = false;
        info!("已禁用代理");
        self.save_config()
    }
    
    /// 保存配置
    pub fn save_config(&self) -> Result<(), AppError> {
        let config_path = self.models_dir.join("proxy_config.json");
        let content = serde_json::to_string_pretty(&self.config)
            .map_err(|e| AppError::ConfigError(format!("序列化代理配置失败: {}", e)))?;
        
        std::fs::write(&config_path, content)
            .map_err(|e| AppError::ConfigError(format!("保存代理配置失败: {}", e)))?;
        
        info!("代理配置已保存到: {}", config_path.display());
        Ok(())
    }
    
    /// 获取用户代理字符串
    pub fn get_user_agent(&self) -> &str {
        &self.config.security_settings.user_agent
    }
    
    /// 检查URL是否应该绕过代理
    pub fn should_bypass_proxy(&self, url: &str) -> bool {
        for pattern in &self.config.proxy_settings.bypass_list {
            if self.matches_pattern(url, pattern) {
                debug!("URL {} 匹配绕过规则: {}", url, pattern);
                return true;
            }
        }
        false
    }
    
    /// 简单的模式匹配
    fn matches_pattern(&self, url: &str, pattern: &str) -> bool {
        if pattern.starts_with("*.") {
            let domain = &pattern[2..];
            url.contains(domain)
        } else {
            url.contains(pattern)
        }
    }
}

/// 代理信息结构
#[derive(Debug, Clone)]
pub struct ProxyInfo {
    pub proxy_type: ProxyType,
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl ProxyInfo {
    /// 构建代理URL
    pub fn to_url(&self) -> String {
        let auth = if let (Some(username), Some(password)) = (&self.username, &self.password) {
            format!("{}:{}@", username, password)
        } else if let Some(username) = &self.username {
            format!("{}@", username)
        } else {
            String::new()
        };
        
        format!("{}://{}{}:{}", 
                self.proxy_type.to_string(), 
                auth, 
                self.host, 
                self.port)
    }
} 