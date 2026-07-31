//! 东方财富外汇数据结构模型定义

use serde::{Deserialize, Serialize};

/// 外汇查询参数
#[derive(Debug, Deserialize, Serialize)]
pub struct ForexQuery {
    pub symbol: Option<String>,
}
