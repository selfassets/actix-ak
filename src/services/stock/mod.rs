//! 股票数据服务模块
//!
//! 提供股票相关的数据服务，支持多种数据源

pub mod sina;

// 重新导出常用函数，保持对外接口一致
pub use sina::{get_stock_info, get_stock_history, list_stocks};
