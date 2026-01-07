//! 期货数据服务
//!
//! 提供期货数据的获取和处理逻辑，参考 akshare 实现
//!
//! ## 数据来源
//! - 新浪财经：实时行情、K线数据、持仓排名
//! - 100ppi：现货价格及基差数据
//! - 99期货网：库存数据
//! - OpenCTP：交易费用数据
//! - 国泰君安：交易规则数据
//!
//! ## 主要功能
//! - 期货实时行情获取
//! - 日K线/分钟K线数据
//! - 品种映射和交易所信息
//! - 主力连续合约数据
//! - 持仓排名数据
//! - 外盘期货数据
//! - 现货价格及基差
//! - 交易费用和规则

#![allow(dead_code)]
#![allow(unused_imports)]

mod common;
mod fees;
mod foreign;
mod inventory;
mod kline;
mod main_contract;
mod position_rank;
mod sina;
mod spot;
mod warehouse;

// 重新导出公共类型和函数（这些是公共 API，供外部使用）
pub use common::get_beijing_time;
pub use fees::{get_futures_comm_info, get_futures_fees_info, get_futures_rule};
pub use foreign::{
    get_foreign_futures_realtime, get_foreign_futures_symbols, get_futures_foreign_detail,
    get_futures_foreign_hist,
};
pub use inventory::{get_99_symbol_map, get_futures_inventory_99};
pub use kline::{get_futures_history, get_futures_minute_data};
pub use main_contract::{
    get_futures_display_main_sina, get_futures_hold_pos_sina, get_futures_main_sina,
};
pub use sina::FuturesService;
pub use spot::{
    get_futures_spot_price, get_futures_spot_price_daily, get_futures_spot_price_previous,
};

// 持仓排名相关（公共 API，暂未在 handlers 中使用）
pub use position_rank::{
    futures_dce_position_rank, futures_dce_position_rank_other, futures_gfex_position_rank,
    futures_hold_pos_sina as futures_hold_pos_sina_rank, get_cffex_rank_table, get_dce_rank_table,
    get_gfex_rank_table, get_gfex_vars_list, get_rank_sum, get_rank_sum_daily,
    get_rank_table_czce, get_shfe_rank_table,
};

// 仓单日报相关（公共 API，暂未在 handlers 中使用）
pub use warehouse::{
    futures_gfex_warehouse_receipt, futures_shfe_warehouse_receipt,
    futures_warehouse_receipt_czce, futures_warehouse_receipt_dce,
};
