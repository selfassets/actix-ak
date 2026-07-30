//! 计算与统计模型 (Cal)

use serde::{Deserialize, Serialize};

/// OHLC K 线数据数据项（输入）
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct OhlcItem {
    pub date: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
}

/// Yang-Zhang 已实现波动率计算结果
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct YangZhangVolatilityResult {
    /// 总体 YZ 已实现波动率 (年化或周期计算值)
    pub yang_zhang_volatility: f64,
    /// 样本数量
    pub count: usize,
    /// 隔夜波动率分量 Vo
    pub vo: f64,
    /// 收盘波动率分量 Vc
    pub vc: f64,
    /// Rogers-Satchell 波动率分量 Vrs
    pub vrs: f64,
    /// 权重 k
    pub k: f64,
}

/// 分钟 K 线清洗数据查询参数
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::IntoParams, utoipa::ToSchema))]
pub struct RvMinuteQuery {
    /// 证券/期货合约代码，例如 "000001" 或 "IF2008"
    pub symbol: Option<String>,
    /// 分钟周期，选择范围 {'1','5','15','30','60'}，默认 "5"
    pub period: Option<String>,
    /// 开始日期时间，格式 YYYY-MM-DD HH:MM:SS 或 YYYYMMDD
    pub start_date: Option<String>,
    /// 结束日期时间，格式 YYYY-MM-DD HH:MM:SS 或 YYYYMMDD
    pub end_date: Option<String>,
    /// 复权类型，选择范围 {'','qfq','hfq'}，默认 "hfq"
    pub adjust: Option<String>,
}
