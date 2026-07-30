//! 加密货币数据模型

use serde::{Deserialize, Serialize};

/// 加密货币查询参数
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::IntoParams, utoipa::ToSchema))]
pub struct CryptoQuery {
    /// 查询日期，格式 YYYYMMDD，默认 "20230830"
    pub date: Option<String>,
}

/// CME 比特币成交量及未平仓合约报告条目
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct CryptoBitcoinCmeItem {
    /// 动态包含电子交易合约、场内成交合约、场外成交合约、成交量、未平仓合约、持仓变化等
    #[serde(flatten)]
    pub data: std::collections::HashMap<String, serde_json::Value>,
}

/// 全球机构/上市公司比特币持仓报告条目
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct CryptoBitcoinHoldItem {
    /// 代码 / 股票 Ticker
    pub symbol: Option<String>,
    /// 公司英文名称
    pub name_en: Option<String>,
    /// 公司中文名称
    pub name_cn: Option<String>,
    /// 国家/地区
    pub country: Option<String>,
    /// 市值
    pub market_cap: Option<f64>,
    /// 比特币占市值比重
    pub btc_market_ratio: Option<f64>,
    /// 持仓成本
    pub cost: Option<f64>,
    /// 持仓占比
    pub hold_ratio: Option<f64>,
    /// 持仓量
    pub hold_amount: Option<f64>,
    /// 当日持仓市值
    pub current_hold_market_val: Option<f64>,
    /// 查询日期
    pub date: Option<String>,
    /// 公告链接
    pub link: Option<String>,
    /// 分类
    pub category: Option<String>,
    /// 倍数
    pub multiple: Option<f64>,
}
