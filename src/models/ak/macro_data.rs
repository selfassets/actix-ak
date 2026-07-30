//! 宏观经济数据模型

use serde::{Deserialize, Serialize};

/// 通用宏观数据查询参数
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::IntoParams, utoipa::ToSchema))]
pub struct MacroQuery {
    /// 变体或标的代码（如 "yearly", "monthly", "cx" 等）
    pub symbol: Option<String>,
}

/// 宏观数据记录通用条目（键值映射表）
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct MacroItem {
    /// 动态包含响应中的时间、指标数值、同比/环比增长率等字段
    #[serde(flatten)]
    pub data: std::collections::HashMap<String, serde_json::Value>,
}
