//! 认证模块
//!
//! 用户存储、密码哈希、JWT 签发与验证

use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// JWT Claims
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// 用户名
    pub sub: String,
    /// 角色
    pub role: String,
    /// 过期时间（UNIX 时间戳）
    pub exp: usize,
    /// 签发时间
    pub iat: usize,
}

/// 用户信息
#[derive(Debug, Clone)]
pub struct User {
    pub username: String,
    pub password_hash: String,
    pub role: String,
}

/// 线程安全的用户存储
#[derive(Debug, Clone)]
pub struct UserStore {
    users: Arc<RwLock<HashMap<String, User>>>,
}

impl UserStore {
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 添加用户（密码将被哈希存储）
    pub async fn add_user(&self, username: &str, password: &str, role: &str) -> Result<(), String> {
        let mut users = self.users.write().await;
        if users.contains_key(username) {
            return Err(format!("用户已存在: {}", username));
        }

        let password_hash =
            bcrypt::hash(password, bcrypt::DEFAULT_COST).map_err(|e| e.to_string())?;

        users.insert(
            username.to_string(),
            User {
                username: username.to_string(),
                password_hash,
                role: role.to_string(),
            },
        );

        log::info!("新用户注册: {} ({})", username, role);
        Ok(())
    }

    /// 验证用户凭据
    pub async fn verify(&self, username: &str, password: &str) -> Option<User> {
        let users = self.users.read().await;
        if let Some(user) = users.get(username) {
            if bcrypt::verify(password, &user.password_hash).unwrap_or(false) {
                return Some(user.clone());
            }
        }
        None
    }
}

/// 签发 JWT Token
pub fn create_token(
    username: &str,
    role: &str,
    secret: &str,
    expire_hours: u64,
) -> Result<String, String> {
    let now = Utc::now().timestamp() as usize;
    let claims = Claims {
        sub: username.to_string(),
        role: role.to_string(),
        exp: now + (expire_hours as usize * 3600),
        iat: now,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| e.to_string())
}

/// 验证 JWT Token
pub fn validate_token(token: &str, secret: &str) -> Result<Claims, String> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| e.to_string())
}
