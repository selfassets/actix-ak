use serde::{Deserialize, Serialize};
use chrono::Utc;
use chrono_tz::Asia::Shanghai;

// 获取北京时间
fn get_beijing_time() -> chrono::DateTime<chrono_tz::Tz> {
    Utc::now().with_timezone(&Shanghai)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: String,
    pub timestamp: String,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: "Success".to_string(),
            timestamp: get_beijing_time().to_rfc3339(),
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            message,
            timestamp: get_beijing_time().to_rfc3339(),
        }
    }
}