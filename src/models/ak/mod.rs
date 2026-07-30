//! AkShare (AK) 数据模型
//!
//! 定义 AK 模块相关的请求和响应结构

pub mod bank;
pub mod bond;
pub mod cal;
pub mod crypto;
pub mod currency;
pub mod interest_rate;
pub mod macro_data;

use serde::{Deserialize, Serialize};

/// AK 接口元数据信息
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct AkInfo {
    /// 模块名称
    pub name: String,
    /// 模块版本
    pub version: String,
    /// 描述信息
    pub description: String,
    /// 支持的接口数量或分类
    pub categories: Vec<String>,
}

/// AK 接口查询参数
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::IntoParams, utoipa::ToSchema))]
pub struct AkQuery {
    /// 接口/数据分类名称
    pub category: Option<String>,
}

/// 经济政策不确定性指数查询参数
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::IntoParams, utoipa::ToSchema))]
pub struct EpuIndexQuery {
    /// 指定的国家或地区名称，默认 "China"（例如 "China", "USA", "Japan" 等）
    pub symbol: Option<String>,
}

/// 经济政策不确定性指数数据条目
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct EpuIndexItem {
    /// 年份
    pub year: Option<i32>,
    /// 月份
    pub month: Option<i32>,
    /// 指数值
    pub epu: Option<f64>,
    /// 原始行的其它数据/字段键值映射（兼容各种 CSV/Excel 列名结构）
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// FRED 宏观经济数据查询参数
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::IntoParams, utoipa::ToSchema))]
pub struct FredQuery {
    /// 年月字符串，默认 "2020-01"（例如 "2020-03", "2023-01" 等）
    pub date: Option<String>,
}

/// FRED 宏观经济数据记录条目
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct FredItem {
    /// 动态包含 CSV 的所有列字段及对应值
    #[serde(flatten)]
    pub data: std::collections::HashMap<String, serde_json::Value>,
}

/// Oxford-Man 实际波动率查询参数
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::IntoParams, utoipa::ToSchema))]
pub struct OmanRvQuery {
    /// 代码，如 "FTSE", "SPX", "SSEC" 等（默认 "FTSE"）
    pub symbol: Option<String>,
    /// 指标类型，如 "rk_th2", "rv5", "rv10" 等（默认 "rk_th2"）
    pub index: Option<String>,
}

/// Oxford-Man 实际波动率简易接口查询参数
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::IntoParams, utoipa::ToSchema))]
pub struct OmanRvShortQuery {
    /// 代码，如 "FTSE", "SPX", "SSEC" 等（默认 "FTSE"）
    pub symbol: Option<String>,
}

/// Risk Lab 实际波动率查询参数
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::IntoParams, utoipa::ToSchema))]
pub struct RlabRvQuery {
    /// 股票/品种代码，默认 "39693"
    pub symbol: Option<String>,
}

/// 波动率通用时间序列条目
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct VolatilityItem {
    /// 日期或时间戳描述
    pub date: String,
    /// 波动率数值
    pub value: Option<f64>,
}
