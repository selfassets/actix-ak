//! 利率与银行间拆借利率数据模型

use serde::{Deserialize, Serialize};

/// 银行间拆借利率查询参数
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::IntoParams, utoipa::ToSchema))]
pub struct InterbankRateQuery {
    /// 市场选择：例如 "上海银行同业拆借市场", "伦敦银行同业拆借市场"（默认 "上海银行同业拆借市场"）
    pub market: Option<String>,
    /// 品种选择：例如 "Shibor人民币", "Libor美元", "Hibor港币"（默认 "Shibor人民币"）
    pub symbol: Option<String>,
    /// 期限指标：例如 "隔夜", "1周", "1月", "1年"（默认 "隔夜"）
    pub indicator: Option<String>,
}

/// 拆借利率数据记录条目
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct InterbankRateItem {
    /// 报告日期
    pub date: Option<String>,
    /// 利率数值 %
    pub rate: Option<f64>,
    /// 涨跌 BP / %
    pub change_rate: Option<f64>,
    /// 市场名称
    pub market: Option<String>,
    /// 货币代码
    pub currency: Option<String>,
    /// 其它响应数据键值映射
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}
