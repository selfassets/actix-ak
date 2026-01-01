use serde::{Deserialize, Serialize};

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
    pub updated_at: String,
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

/// 期货查询参数
#[allow(dead_code)]
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