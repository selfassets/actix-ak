//! 配置模块
//!
//! 支持从 JSON 文件加载系统配置，并支持通过环境变量覆盖

use serde::{Deserialize, Serialize};
use std::env;
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
fn default_host() -> String {
    "0.0.0.0".to_string()
}
fn default_port() -> u16 {
    8080
}
fn default_timeout() -> u64 {
    30
}
fn default_connect_timeout() -> u64 {
    10
}
fn default_log_level() -> String {
    "info".to_string()
}

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

    /// 使用环境变量覆盖配置项
    ///
    /// 支持的环境变量：
    /// - `API_KEY`: 覆盖 api.api_key
    /// - `SERVER_HOST`: 覆盖 server.host
    /// - `SERVER_PORT`: 覆盖 server.port
    /// - `SERVER_WORKERS`: 覆盖 server.workers
    /// - `LOG_LEVEL`: 覆盖 log.level
    /// - `TIMEOUT_SECS`: 覆盖 api.timeout_secs
    /// - `CONNECT_TIMEOUT_SECS`: 覆盖 api.connect_timeout_secs
    fn apply_env_overrides(&mut self) {
        // 仅在环境变量存在且非空时才覆盖配置
        if let Ok(val) = env::var("API_KEY") {
            if !val.is_empty() {
                log::info!("使用环境变量 API_KEY 覆盖配置");
                self.api.api_key = val;
            }
        }
        if let Ok(val) = env::var("SERVER_HOST") {
            if !val.is_empty() {
                log::info!("使用环境变量 SERVER_HOST 覆盖配置");
                self.server.host = val;
            }
        }
        if let Ok(val) = env::var("SERVER_PORT") {
            if !val.is_empty() {
                if let Ok(port) = val.parse::<u16>() {
                    log::info!("使用环境变量 SERVER_PORT 覆盖配置");
                    self.server.port = port;
                } else {
                    log::warn!("环境变量 SERVER_PORT 值无效: {}", val);
                }
            }
        }
        if let Ok(val) = env::var("SERVER_WORKERS") {
            if !val.is_empty() {
                if let Ok(workers) = val.parse::<usize>() {
                    log::info!("使用环境变量 SERVER_WORKERS 覆盖配置");
                    self.server.workers = workers;
                } else {
                    log::warn!("环境变量 SERVER_WORKERS 值无效: {}", val);
                }
            }
        }
        if let Ok(val) = env::var("LOG_LEVEL") {
            if !val.is_empty() {
                log::info!("使用环境变量 LOG_LEVEL 覆盖配置");
                self.log.level = val;
            }
        }
        if let Ok(val) = env::var("TIMEOUT_SECS") {
            if !val.is_empty() {
                if let Ok(secs) = val.parse::<u64>() {
                    log::info!("使用环境变量 TIMEOUT_SECS 覆盖配置");
                    self.api.timeout_secs = secs;
                } else {
                    log::warn!("环境变量 TIMEOUT_SECS 值无效: {}", val);
                }
            }
        }
        if let Ok(val) = env::var("CONNECT_TIMEOUT_SECS") {
            if !val.is_empty() {
                if let Ok(secs) = val.parse::<u64>() {
                    log::info!("使用环境变量 CONNECT_TIMEOUT_SECS 覆盖配置");
                    self.api.connect_timeout_secs = secs;
                } else {
                    log::warn!("环境变量 CONNECT_TIMEOUT_SECS 值无效: {}", val);
                }
            }
        }
    }

    /// 加载配置，优先从文件，失败则使用默认值，最后应用环境变量覆盖
    pub fn load() -> Self {
        let config_paths = ["config.json", "config/config.json"];

        let mut config = None;
        for path in config_paths {
            if Path::new(path).exists() {
                match Self::from_file(path) {
                    Ok(c) => {
                        log::info!("从 {} 加载配置成功", path);
                        config = Some(c);
                        break;
                    }
                    Err(e) => {
                        log::warn!("加载配置文件 {} 失败: {}", path, e);
                    }
                }
            }
        }

        let mut config = config.unwrap_or_else(|| {
            log::info!("使用默认配置");
            Self::default()
        });

        // 环境变量覆盖（优先级最高）
        config.apply_env_overrides();
        config
    }

    /// 获取服务器绑定地址
    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }
}
