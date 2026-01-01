//! 配置模块
//!
//! 支持从 JSON 文件加载系统配置

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// 服务器配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// 监听地址
    #[serde(default = "default_host")]
    pub host: String,
    /// 监听端口
    #[serde(default = "default_port")]
    pub port: u16,
    /// 工作线程数（0 表示使用 CPU 核心数）
    #[serde(default)]
    pub workers: usize,
}

/// API 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    /// API Key（为空则不启用认证）
    #[serde(default)]
    pub api_key: String,
    /// 请求超时时间（秒）
    #[serde(default = "default_timeout")]
    pub timeout_secs: u64,
    /// 连接超时时间（秒）
    #[serde(default = "default_connect_timeout")]
    pub connect_timeout_secs: u64,
}

/// 日志配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogConfig {
    /// 日志级别: trace, debug, info, warn, error
    #[serde(default = "default_log_level")]
    pub level: String,
}

/// 应用配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// 服务器配置
    #[serde(default)]
    pub server: ServerConfig,
    /// API 配置
    #[serde(default)]
    pub api: ApiConfig,
    /// 日志配置
    #[serde(default)]
    pub log: LogConfig,
}

// 默认值函数
fn default_host() -> String { "0.0.0.0".to_string() }
fn default_port() -> u16 { 8080 }
fn default_timeout() -> u64 { 30 }
fn default_connect_timeout() -> u64 { 10 }
fn default_log_level() -> String { "info".to_string() }

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_port(),
            workers: 0,
        }
    }
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            timeout_secs: default_timeout(),
            connect_timeout_secs: default_connect_timeout(),
        }
    }
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: default_log_level(),
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            api: ApiConfig::default(),
            log: LogConfig::default(),
        }
    }
}

impl AppConfig {
    /// 从 JSON 文件加载配置
    pub fn from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: AppConfig = serde_json::from_str(&content)?;
        Ok(config)
    }

    /// 加载配置，优先从文件，失败则使用默认值
    pub fn load() -> Self {
        let config_paths = ["config.json", "config/config.json"];
        
        for path in config_paths {
            if Path::new(path).exists() {
                match Self::from_file(path) {
                    Ok(config) => {
                        log::info!("从 {} 加载配置成功", path);
                        return config;
                    }
                    Err(e) => {
                        log::warn!("加载配置文件 {} 失败: {}", path, e);
                    }
                }
            }
        }
        
        log::info!("使用默认配置");
        Self::default()
    }

    /// 获取服务器绑定地址
    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }
}
