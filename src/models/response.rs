//! 通用 API 响应模型
//! 
//! 定义统一的 API 响应格式

use serde::{Deserialize, Serialize};
use chrono::Utc;
use chrono_tz::Asia::Shanghai;

/// 获取北京时间（UTC+8）
fn get_beijing_time() -> chrono::DateTime<chrono_tz::Tz> {
    Utc::now().with_timezone(&Shanghai)
}

/// 统一 API 响应结构
/// 
/// 所有接口返回统一格式，包含：
/// - success: 请求是否成功
/// - data: 响应数据（成功时有值）
/// - message: 响应消息
/// - timestamp: 响应时间戳（北京时间）
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    /// 请求是否成功
    pub success: bool,
    /// 响应数据
    pub data: Option<T>,
    /// 响应消息
    pub message: String,
    /// 响应时间戳（ISO 8601 格式）
    pub timestamp: String,
}

impl<T> ApiResponse<T> {
    /// 创建成功响应
    /// 
    /// # 参数
    /// - data: 响应数据
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: "Success".to_string(),
            timestamp: get_beijing_time().to_rfc3339(),
        }
    }

    /// 创建错误响应
    /// 
    /// # 参数
    /// - message: 错误信息
    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            message,
            timestamp: get_beijing_time().to_rfc3339(),
        }
    }
}