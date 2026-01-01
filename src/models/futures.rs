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
    pub exchange: Option<String>, // 交易所：DCE, CZCE, SHFE, INE, CFFEX, GFEX
    pub category: Option<String>, // 品种分类
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub limit: Option<usize>,
}

/// 交易所信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FuturesExchange {
    pub code: String,
    pub name: String,
    pub description: String,
}

/// 期货品种映射信息
/// 对应 akshare 的 futures_symbol_mark() 返回结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FuturesSymbolMark {
    pub exchange: String,      // 交易所名称（中文）
    pub symbol: String,        // 品种名称（如 PTA、铜）
    pub mark: String,          // 新浪API的node参数（如 pta_qh、tong_qh）
}

/// 期货合约详情
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FuturesContractDetail {
    pub symbol: String,              // 合约代码
    pub name: String,                // 合约名称
    pub exchange: String,            // 上市交易所
    pub trading_unit: String,        // 交易单位
    pub quote_unit: String,          // 报价单位
    pub min_price_change: String,    // 最小变动价位
    pub price_limit: String,         // 涨跌停板幅度
    pub contract_months: String,     // 合约交割月份
    pub trading_hours: String,       // 交易时间
    pub last_trading_day: String,    // 最后交易日
    pub last_delivery_day: String,   // 最后交割日
    pub delivery_grade: String,      // 交割品级
    pub margin: String,              // 最低交易保证金
    pub delivery_method: String,     // 交割方式
}

/// 外盘期货品种信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ForeignFuturesSymbol {
    pub symbol: String,   // 品种中文名
    pub code: String,     // 品种代码
}

/// 主力连续合约信息
/// 对应 akshare 的 futures_display_main_sina() 返回结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FuturesMainContract {
    pub symbol: String,   // 合约代码（如 V0, RB0）
    pub name: String,     // 合约名称（如 PVC连续）
    pub exchange: String, // 交易所代码
}

/// 主力连续合约日K线数据
/// 对应 akshare 的 futures_main_sina() 返回结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FuturesMainDailyData {
    pub date: String,           // 日期
    pub open: f64,              // 开盘价
    pub high: f64,              // 最高价
    pub low: f64,               // 最低价
    pub close: f64,             // 收盘价
    pub volume: u64,            // 成交量
    pub hold: u64,              // 持仓量
    pub settle: Option<f64>,    // 动态结算价
}

/// 期货持仓排名数据
/// 对应 akshare 的 futures_hold_pos_sina() 返回结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FuturesHoldPosition {
    pub rank: u32,              // 名次
    pub company: String,        // 期货公司
    pub value: i64,             // 数值（成交量/多单持仓/空单持仓）
    pub change: i64,            // 比上交易日增减
}

/// 持仓排名查询参数
#[derive(Debug, Deserialize)]
pub struct FuturesHoldPosQuery {
    pub pos_type: Option<String>,  // 类型：volume(成交量), long(多单持仓), short(空单持仓)
    pub contract: String,          // 合约代码
    pub date: String,              // 查询日期 YYYYMMDD
}

/// 主力连续日数据查询参数
#[derive(Debug, Deserialize)]
pub struct FuturesMainQuery {
    pub start_date: Option<String>,  // 开始日期 YYYYMMDD
    pub end_date: Option<String>,    // 结束日期 YYYYMMDD
}