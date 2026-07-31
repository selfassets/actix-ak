//! 基金接口数据结构模型

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct FundQuery {
    pub symbol: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FundIndexQuery {
    pub symbol: Option<String>,
    pub indicator: Option<String>,
}
