//! 外汇与货币数据模型

use serde::{Deserialize, Serialize};

/// 中国银行外汇牌价查询参数
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::IntoParams, utoipa::ToSchema))]
pub struct CurrencyBocQuery {
    /// 货币名称选择，例如 "美元", "欧元", "日元", "港币" 等（默认 "美元"）
    pub symbol: Option<String>,
    /// 开始日期 YYYYMMDD（默认 "20230101"）
    pub start_date: Option<String>,
    /// 结束日期 YYYYMMDD（默认当前/近期日期）
    pub end_date: Option<String>,
}

/// 中国银行外汇牌价历史记录条目
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct CurrencyBocItem {
    /// 货币名称
    pub currency: Option<String>,
    /// 汇率中间价 / 汇买价 / 钞买价 / 汇卖价 / 钞卖价等
    pub date: Option<String>,
    /// 中买价
    pub bank_conversion_pri: Option<f64>,
    /// 钞买价
    pub bank_cash_buy_pri: Option<f64>,
    /// 汇买价
    pub bank_foreign_buy_pri: Option<f64>,
    /// 钞卖价
    pub bank_cash_sell_pri: Option<f64>,
    /// 汇卖价
    pub bank_foreign_sell_pri: Option<f64>,
    /// 发布时间
    pub publish_time: Option<String>,
}

/// 国家外汇管理局 (SAFE) 人民币汇率中间价条目
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct CurrencySafeItem {
    /// 动态包含 SAFE 各种货币的中间价
    #[serde(flatten)]
    pub data: std::collections::HashMap<String, serde_json::Value>,
}
