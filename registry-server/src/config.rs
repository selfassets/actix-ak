//! 注册中心配置模块

use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;

/// 服务器配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
}

/// 注册中心配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryConfig {
    /// 心跳超时阈值（秒）
    #[serde(default = "default_heartbeat_timeout")]
    pub heartbeat_timeout_secs: u64,
}

/// 日志配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogConfig {
    #[serde(default = "default_log_level")]
    pub level: String,
}

/// 认证配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    /// JWT 密钥
    #[serde(default = "default_jwt_secret")]
    pub jwt_secret: String,
    /// 默认管理员用户名
    #[serde(default = "default_admin_username")]
    pub admin_username: String,
    /// 默认管理员密码
    #[serde(default = "default_admin_password")]
    pub admin_password: String,
    /// Token 过期时间（小时）
    #[serde(default = "default_token_expire_hours")]
    pub token_expire_hours: u64,
}

/// 应用配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub server: ServerConfig,
    #[serde(default)]
    pub registry: RegistryConfig,
    #[serde(default)]
    pub log: LogConfig,
    #[serde(default)]
    pub auth: AuthConfig,
}

fn default_host() -> String {
    "0.0.0.0".to_string()
}
fn default_port() -> u16 {
    8081
}
fn default_heartbeat_timeout() -> u64 {
    30
}
fn default_log_level() -> String {
    "info".to_string()
}
fn default_jwt_secret() -> String {
    "registry-server-secret-key".to_string()
}
fn default_admin_username() -> String {
    "admin".to_string()
}
fn default_admin_password() -> String {
    "admin123".to_string()
}
fn default_token_expire_hours() -> u64 {
    24
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_port(),
        }
    }
}

impl Default for RegistryConfig {
    fn default() -> Self {
        Self {
            heartbeat_timeout_secs: default_heartbeat_timeout(),
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

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            jwt_secret: default_jwt_secret(),
            admin_username: default_admin_username(),
            admin_password: default_admin_password(),
            token_expire_hours: default_token_expire_hours(),
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            registry: RegistryConfig::default(),
            log: LogConfig::default(),
            auth: AuthConfig::default(),
        }
    }
}

impl AppConfig {
    pub fn load() -> Self {
        // 支持从 workspace 根目录或 registry-server 目录运行
        let config_paths = ["registry-server/config.json", "config.json"];

        let mut config = None;
        for path in config_paths {
            if Path::new(path).exists() {
                match fs::read_to_string(path) {
                    Ok(content) => {
                        // 跳过不属于 registry-server 的配置文件（如 actix-ak 的 config.json）
                        if !content.contains("heartbeat_timeout_secs") {
                            log::debug!("跳过非注册中心配置文件: {}", path);
                            continue;
                        }
                        match serde_json::from_str(&content) {
                            Ok(c) => {
                                log::info!("从 {} 加载配置成功", path);
                                config = Some(c);
                                break;
                            }
                            Err(e) => log::warn!("解析配置文件 {} 失败: {}", path, e),
                        }
                    }
                    Err(e) => log::warn!("读取配置文件 {} 失败: {}", path, e),
                }
            }
        }

        let mut config = config.unwrap_or_else(|| {
            log::info!("使用默认配置");
            Self::default()
        });

        // 环境变量覆盖
        if let Ok(val) = env::var("SERVER_HOST") {
            if !val.is_empty() {
                config.server.host = val;
            }
        }
        if let Ok(val) = env::var("SERVER_PORT") {
            if let Ok(port) = val.parse::<u16>() {
                config.server.port = port;
            }
        }
        if let Ok(val) = env::var("HEARTBEAT_TIMEOUT_SECS") {
            if let Ok(secs) = val.parse::<u64>() {
                config.registry.heartbeat_timeout_secs = secs;
            }
        }
        if let Ok(val) = env::var("LOG_LEVEL") {
            if !val.is_empty() {
                config.log.level = val;
            }
        }
        // 认证相关环境变量
        if let Ok(val) = env::var("JWT_SECRET") {
            if !val.is_empty() {
                config.auth.jwt_secret = val;
            }
        }
        if let Ok(val) = env::var("ADMIN_USERNAME") {
            if !val.is_empty() {
                config.auth.admin_username = val;
            }
        }
        if let Ok(val) = env::var("ADMIN_PASSWORD") {
            if !val.is_empty() {
                config.auth.admin_password = val;
            }
        }
        if let Ok(val) = env::var("TOKEN_EXPIRE_HOURS") {
            if let Ok(hours) = val.parse::<u64>() {
                config.auth.token_expire_hours = hours;
            }
        }

        config
    }

    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }
}
