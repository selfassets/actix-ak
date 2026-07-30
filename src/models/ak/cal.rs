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
