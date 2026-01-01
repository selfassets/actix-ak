use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FuturesInfo {
    pub symbol: String,
    pub name: String,
    pub current_price: f64,
    pub change: f64,
    pub change_percent: f64,
    pub volume: u64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub settlement: Option<f64>,
    pub prev_settlement: Option<f64>,
    pub open_interest: Option<u64>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FuturesHistoryData {
    pub symbol: String,
    pub date: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u64,
    pub settlement: Option<f64>,
    pub open_interest: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct FuturesQuery {
    pub symbol: Option<String>,
    pub exchange: Option<String>, // 交易所：DCE, CZCE, SHFE, INE
    pub category: Option<String>, // 品种分类
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FuturesExchange {
    pub code: String,
    pub name: String,
    pub description: String,
}

// 新浪期货数据原始响应结构
#[derive(Debug, Deserialize)]
pub struct SinaFuturesResponse {
    pub result: SinaFuturesResult,
}

#[derive(Debug, Deserialize)]
pub struct SinaFuturesResult {
    pub status: SinaStatus,
    pub data: Vec<SinaFuturesData>,
}

#[derive(Debug, Deserialize)]
pub struct SinaStatus {
    pub code: i32,
    pub msg: String,
}

#[derive(Debug, Deserialize)]
pub struct SinaFuturesData {
    pub symbol: String,
    pub name: String,
    pub current: String,
    pub change: String,
    pub percent: String,
    pub volume: String,
    pub open: String,
    pub high: String,
    pub low: String,
    pub settlement: Option<String>,
    pub prev_settlement: Option<String>,
    pub open_interest: Option<String>,
}