//! AkShare (AK) 数据模型
//!
//! 定义 AK 模块相关的请求和响应结构

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
