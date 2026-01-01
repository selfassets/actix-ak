//! 股票数据模型
//! 
//! 定义股票相关的数据结构

use serde::{Deserialize, Serialize};

/// 股票基本信息
/// 
/// 包含股票的实时行情数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StockInfo {
    /// 股票代码
    pub symbol: String,
    /// 股票名称
    pub name: String,
    /// 当前价格
    pub current_price: f64,
    /// 涨跌额
    pub change: f64,
    /// 涨跌幅（百分比）
    pub change_percent: f64,
    /// 成交量
    pub volume: u64,
    /// 市值（可选）
    pub market_cap: Option<f64>,
    /// 更新时间
    pub updated_at: String,
}

/// 股票历史K线数据
/// 
/// 包含单日的 OHLCV 数据
#[derive(Debug, Serialize, Deserialize)]
pub struct StockHistoryData {
    /// 股票代码
    pub symbol: String,
    /// 日期
    pub date: String,
    /// 开盘价
    pub open: f64,
    /// 最高价
    pub high: f64,
    /// 最低价
    pub low: f64,
    /// 收盘价
    pub close: f64,
    /// 成交量
    pub volume: u64,
}

/// 股票查询参数
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct StockQuery {
    /// 股票代码
    pub symbol: Option<String>,
    /// 开始日期（YYYYMMDD）
    pub start_date: Option<String>,
    /// 结束日期（YYYYMMDD）
    pub end_date: Option<String>,
    /// 返回数量限制
    pub limit: Option<usize>,
}