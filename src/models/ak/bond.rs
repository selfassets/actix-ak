//! 债券数据模型
//!
//! 定义可转债、国债收益率、债券回购等请求与响应数据结构

use serde::{Deserialize, Serialize};

/// 债券查询参数
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::IntoParams, utoipa::ToSchema))]
pub struct BondQuery {
    /// 债券代码或分类标志，例如 "113527"、"中国10年期国债" 或 "204001"
    pub symbol: Option<String>,
    /// 开始日期，格式 YYYYMMDD
    pub start_date: Option<String>,
    /// 结束日期，格式 YYYYMMDD
    pub end_date: Option<String>,
}

/// 沪深可转债实时行情条目
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct BondZhCovSpotItem {
    /// 可转债代码
    pub code: Option<String>,
    /// 可转债简称
    pub name: Option<String>,
    /// 现价
    pub trade: Option<f64>,
    /// 涨跌额
    pub change_price: Option<f64>,
    /// 涨跌幅 %
    pub change_percent: Option<f64>,
    /// 动态包含全部响应字段
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// 国债收益率 K 线 / 历史行情数据条目
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct BondGbKlineItem {
    /// 日期
    pub date: Option<String>,
    /// 开盘价/收益率
    pub open: Option<f64>,
    /// 最高价/收益率
    pub high: Option<f64>,
    /// 最低价/收益率
    pub low: Option<f64>,
    /// 收盘价/收益率
    pub close: Option<f64>,
    /// 成交量
    pub volume: Option<f64>,
}

/// 中美国债收益率对比数据条目
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct BondZhUsRateItem {
    /// 日期
    pub date: Option<String>,
    /// 中国1年期国债收益率
    pub cn_1y: Option<f64>,
    /// 中国10年期国债收益率
    pub cn_10y: Option<f64>,
    /// 美国1年期国债收益率
    pub us_1y: Option<f64>,
    /// 美国10年期国债收益率
    pub us_10y: Option<f64>,
    /// 10年期中美利差
    pub spread_10y: Option<f64>,
    /// 其它详细字段
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// 质押式债券回购行情数据条目
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct BondBuyBackItem {
    /// 代码
    pub code: Option<String>,
    /// 名称
    pub name: Option<String>,
    /// 最新价（收益率 %）
    pub price: Option<f64>,
    /// 涨跌额
    pub change_price: Option<f64>,
    /// 涨跌幅 %
    pub change_percent: Option<f64>,
    /// 今开
    pub open: Option<f64>,
    /// 最高
    pub high: Option<f64>,
    /// 最低
    pub low: Option<f64>,
    /// 昨收
    pub close: Option<f64>,
    /// 成交量
    pub volume: Option<f64>,
    /// 成交额
    pub amount: Option<f64>,
}

/// 集思录可转债指数与强赎等通用数据条目
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct BondJslItem {
    /// 动态包含集思录 JSON 的所有字段映射
    #[serde(flatten)]
    pub data: std::collections::HashMap<String, serde_json::Value>,
}

/// 可转债详情/基本信息项条目
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct BondCbProfileItem {
    /// 项目/指标名称
    pub item: String,
    /// 对应数值/描述
    pub value: String,
}

/// 可转债比价表数据条目
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct BondCovComparisonItem {
    /// 动态包含东方财富可转债比价表的所有字段映射
    #[serde(flatten)]
    pub data: std::collections::HashMap<String, serde_json::Value>,
}
