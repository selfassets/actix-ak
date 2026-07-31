//! 能源接口模型定义

use serde::{Deserialize, Serialize};

/// 原油油价查询参数
#[derive(Debug, Deserialize, Serialize)]
pub struct EnergyOilQuery {
    /// 日期, 默认 "20220517" 等
    pub date: Option<String>,
    pub symbol: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct EnergyItem {
    pub date: String,
    pub symbol: String,
    pub price: Option<f64>,
    pub change: Option<f64>,
    pub volume: Option<f64>,
    pub amount: Option<f64>,
    pub city: Option<String>,
}
