//! 富豪榜结构模型定义

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct FortuneRankQuery {
    /// 年份，如 "2023" 等
    pub year: Option<String>,
    pub symbol: Option<String>,
    pub indicator: Option<String>,
}
