//! 期货数据模型
//! 
//! 定义期货相关的数据结构，包括：
//! - 期货实时行情
//! - 历史K线数据
//! - 交易所和品种信息
//! - 持仓排名数据
//! - 现货价格及基差数据

#![allow(dead_code)]

use serde::{Deserialize, Serialize};

/// 期货合约实时行情
/// 
/// 包含期货合约的实时交易数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FuturesInfo {
    /// 合约代码（如 RB2510）
    pub symbol: String,
    /// 合约名称
    pub name: String,
    /// 当前价格/最新价
    pub current_price: f64,
    /// 涨跌额
    pub change: f64,
    /// 涨跌幅（百分比）
    pub change_percent: f64,
    /// 成交量（手）
    pub volume: u64,
    /// 开盘价
    pub open: f64,
    /// 最高价
    pub high: f64,
    /// 最低价
    pub low: f64,
    /// 结算价
    pub settlement: Option<f64>,
    /// 昨结算价
    pub prev_settlement: Option<f64>,
    /// 持仓量（手）
    pub open_interest: Option<u64>,
    /// 更新时间
    pub updated_at: String,
}

/// 期货历史K线数据
/// 
/// 包含单日的 OHLCV 数据及持仓量
#[derive(Debug, Serialize, Deserialize)]
pub struct FuturesHistoryData {
    /// 合约代码
    pub symbol: String,
    /// 日期
    pub date: String,
    /// 开盘价
    pub open: f64,
    /// 最高价
    pub high: f64,
    /// 最低价
    pub low: f64,
    /// 收盘价
    pub close: f64,
    /// 成交量（手）
    pub volume: u64,
    /// 结算价
    pub settlement: Option<f64>,
    /// 持仓量（手）
    pub open_interest: Option<u64>,
}

/// 期货查询参数
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct FuturesQuery {
    /// 合约代码
    pub symbol: Option<String>,
    /// 交易所代码：DCE(大商所), CZCE(郑商所), SHFE(上期所), INE(能源中心), CFFEX(中金所), GFEX(广期所)
    pub exchange: Option<String>,
    /// 品种分类
    pub category: Option<String>,
    /// 开始日期（YYYYMMDD）
    pub start_date: Option<String>,
    /// 结束日期（YYYYMMDD）
    pub end_date: Option<String>,
    /// 返回数量限制
    pub limit: Option<usize>,
}

/// 交易所信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FuturesExchange {
    /// 交易所代码
    pub code: String,
    /// 交易所中文名称
    pub name: String,
    /// 交易所英文名称
    pub description: String,
}

/// 期货品种映射信息
/// 
/// 对应 akshare 的 futures_symbol_mark() 返回结果
/// 用于将品种名称映射到新浪 API 的 node 参数
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FuturesSymbolMark {
    /// 交易所名称（中文）
    pub exchange: String,
    /// 品种名称（如 PTA、铜）
    pub symbol: String,
    /// 新浪 API 的 node 参数（如 pta_qh、tong_qh）
    pub mark: String,
}

/// 期货合约详情
/// 
/// 包含合约的交易规则和参数
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FuturesContractDetail {
    /// 合约代码
    pub symbol: String,
    /// 合约名称
    pub name: String,
    /// 上市交易所
    pub exchange: String,
    /// 交易单位（如 10吨/手）
    pub trading_unit: String,
    /// 报价单位（如 元/吨）
    pub quote_unit: String,
    /// 最小变动价位
    pub min_price_change: String,
    /// 涨跌停板幅度
    pub price_limit: String,
    /// 合约交割月份
    pub contract_months: String,
    /// 交易时间
    pub trading_hours: String,
    /// 最后交易日
    pub last_trading_day: String,
    /// 最后交割日
    pub last_delivery_day: String,
    /// 交割品级
    pub delivery_grade: String,
    /// 最低交易保证金
    pub margin: String,
    /// 交割方式
    pub delivery_method: String,
}

/// 外盘期货品种信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ForeignFuturesSymbol {
    /// 品种中文名
    pub symbol: String,
    /// 品种代码
    pub code: String,
}

/// 主力连续合约信息
/// 
/// 对应 akshare 的 futures_display_main_sina() 返回结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FuturesMainContract {
    /// 合约代码（如 V0, RB0）
    pub symbol: String,
    /// 合约名称（如 PVC连续）
    pub name: String,
    /// 交易所代码
    pub exchange: String,
}

/// 主力连续合约日K线数据
/// 
/// 对应 akshare 的 futures_main_sina() 返回结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FuturesMainDailyData {
    /// 日期
    pub date: String,
    /// 开盘价
    pub open: f64,
    /// 最高价
    pub high: f64,
    /// 最低价
    pub low: f64,
    /// 收盘价
    pub close: f64,
    /// 成交量（手）
    pub volume: u64,
    /// 持仓量（手）
    pub hold: u64,
    /// 动态结算价
    pub settle: Option<f64>,
}

/// 期货持仓排名数据
/// 
/// 对应 akshare 的 futures_hold_pos_sina() 返回结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FuturesHoldPosition {
    /// 名次
    pub rank: u32,
    /// 期货公司名称
    pub company: String,
    /// 数值（成交量/多单持仓/空单持仓）
    pub value: i64,
    /// 比上交易日增减
    pub change: i64,
}

/// 持仓排名查询参数
#[derive(Debug, Deserialize)]
pub struct FuturesHoldPosQuery {
    /// 类型：volume(成交量), long(多单持仓), short(空单持仓)
    pub pos_type: Option<String>,
    /// 合约代码（如 RB2510）
    pub contract: String,
    /// 查询日期（YYYYMMDD）
    pub date: String,
}

/// 主力连续日数据查询参数
#[derive(Debug, Deserialize)]
pub struct FuturesMainQuery {
    /// 开始日期（YYYYMMDD）
    pub start_date: Option<String>,
    /// 结束日期（YYYYMMDD）
    pub end_date: Option<String>,
}

/// 外盘期货历史数据
/// 
/// 对应 akshare 的 futures_foreign_hist() 返回结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ForeignFuturesHistData {
    /// 日期
    pub date: String,
    /// 开盘价
    pub open: f64,
    /// 最高价
    pub high: f64,
    /// 最低价
    pub low: f64,
    /// 收盘价
    pub close: f64,
    /// 成交量
    pub volume: u64,
}

/// 外盘期货合约详情
/// 对应 akshare 的 futures_foreign_detail() 返回结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ForeignFuturesDetail {
    pub items: Vec<ForeignFuturesDetailItem>,  // 合约详情项列表
}

/// 外盘期货合约详情项
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ForeignFuturesDetailItem {
    pub name: String,           // 项目名称
    pub value: String,          // 项目值
}

/// 期货手续费信息
/// 对应 akshare 的 futures_comm_info() 返回结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FuturesCommInfo {
    pub exchange: String,                    // 交易所名称
    pub contract_name: String,               // 合约名称
    pub contract_code: String,               // 合约代码
    pub current_price: Option<f64>,          // 现价
    pub limit_up: Option<f64>,               // 涨停板
    pub limit_down: Option<f64>,             // 跌停板
    pub margin_buy: Option<f64>,             // 保证金-买开(%)
    pub margin_sell: Option<f64>,            // 保证金-卖开(%)
    pub margin_per_lot: Option<f64>,         // 保证金-每手(元)
    pub fee_open_ratio: Option<f64>,         // 手续费标准-开仓-万分之
    pub fee_open_yuan: Option<f64>,          // 手续费标准-开仓-元
    pub fee_close_yesterday_ratio: Option<f64>,  // 手续费标准-平昨-万分之
    pub fee_close_yesterday_yuan: Option<f64>,   // 手续费标准-平昨-元
    pub fee_close_today_ratio: Option<f64>,      // 手续费标准-平今-万分之
    pub fee_close_today_yuan: Option<f64>,       // 手续费标准-平今-元
    pub profit_per_tick: Option<f64>,        // 每跳毛利
    pub fee_total: Option<f64>,              // 手续费(开+平)
    pub net_profit_per_tick: Option<f64>,    // 每跳净利
    pub remark: Option<String>,              // 备注
}

/// 期货手续费查询参数
#[derive(Debug, Deserialize)]
pub struct FuturesCommQuery {
    pub exchange: Option<String>,  // 交易所：所有/上海期货交易所/大连商品交易所/郑州商品交易所/上海国际能源交易中心/中国金融期货交易所/广州期货交易所
}

/// 期货交易规则信息
/// 对应 akshare 的 futures_rule() 返回结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FuturesRule {
    pub exchange: String,              // 交易所
    pub product: String,               // 品种
    pub code: String,                  // 代码
    pub margin_rate: Option<f64>,      // 交易保证金比例(%)
    pub price_limit: Option<f64>,      // 涨跌停板幅度(%)
    pub contract_size: Option<f64>,    // 合约乘数
    pub price_tick: Option<f64>,       // 最小变动价位
    pub max_order_size: Option<u64>,   // 限价单每笔最大下单手数
    pub special_note: Option<String>,  // 特殊合约参数调整
    pub remark: Option<String>,        // 调整备注
}

/// 期货交易规则查询参数
#[derive(Debug, Deserialize)]
pub struct FuturesRuleQuery {
    pub date: Option<String>,  // 交易日期 YYYYMMDD
}

/// 期货交易费用信息
/// 对应 akshare 的 futures_fees_info() 返回结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FuturesFeesInfo {
    pub exchange: String,              // 交易所
    pub contract_code: String,         // 合约代码
    pub contract_name: String,         // 合约名称
    pub product_code: String,          // 品种代码
    pub product_name: String,          // 品种名称
    pub contract_size: String,         // 合约乘数
    pub price_tick: String,            // 最小跳动
    pub open_fee_rate: String,         // 开仓费率
    pub open_fee: String,              // 开仓费用/手
    pub close_fee_rate: String,        // 平仓费率
    pub close_fee: String,             // 平仓费用/手
    pub close_today_fee_rate: String,  // 平今费率
    pub close_today_fee: String,       // 平今费用/手
    pub long_margin_rate: String,      // 做多保证金率
    pub short_margin_rate: String,     // 做空保证金率
    pub updated_at: String,            // 更新时间
}


/// 99期货网品种信息
/// 用于品种代码映射
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Futures99Symbol {
    pub product_id: i64,      // 品种ID
    pub name: String,         // 品种名称（中文）
    pub code: String,         // 品种代码
}

/// 99期货网库存数据
/// 对应 akshare 的 futures_inventory_99() 返回结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FuturesInventory99 {
    pub date: String,         // 日期
    pub close_price: Option<f64>,  // 收盘价
    pub inventory: Option<f64>,    // 库存
}

/// 99期货网库存查询参数
#[derive(Debug, Deserialize)]
pub struct FuturesInventory99Query {
    pub symbol: String,  // 品种名称或代码，如"豆一"或"A"
}

/// 期货现货价格及基差数据
/// 对应 akshare 的 futures_spot_price() 返回结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FuturesSpotPrice {
    pub date: String,                    // 日期 YYYYMMDD
    pub symbol: String,                  // 品种代码
    pub spot_price: f64,                 // 现货价格
    pub near_contract: String,           // 临近交割合约
    pub near_contract_price: f64,        // 临近交割合约结算价
    pub dominant_contract: String,       // 主力合约
    pub dominant_contract_price: f64,    // 主力合约结算价
    pub near_basis: f64,                 // 临近交割合约相对现货的基差
    pub dom_basis: f64,                  // 主力合约相对现货的基差
    pub near_basis_rate: f64,            // 临近交割合约相对现货的基差率
    pub dom_basis_rate: f64,             // 主力合约相对现货的基差率
}

/// 期货现货价格查询参数
#[derive(Debug, Deserialize)]
pub struct FuturesSpotPriceQuery {
    pub date: String,                    // 交易日期 YYYYMMDD
    pub symbols: Option<String>,         // 品种代码列表，逗号分隔，如"RB,CU"，为空时返回所有品种
}


/// 期货现货价格及基差数据（历史版本，包含180日统计）
/// 对应 akshare 的 futures_spot_price_previous() 返回结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FuturesSpotPricePrevious {
    pub commodity: String,               // 商品名称
    pub spot_price: f64,                 // 现货价格
    pub dominant_contract: String,       // 主力合约代码
    pub dominant_price: f64,             // 主力合约价格
    pub basis: f64,                      // 主力合约基差
    pub basis_rate: f64,                 // 主力合约基差率(%)
    pub basis_180d_high: Option<f64>,    // 180日内主力基差最高
    pub basis_180d_low: Option<f64>,     // 180日内主力基差最低
    pub basis_180d_avg: Option<f64>,     // 180日内主力基差平均
}

/// 期货现货价格历史查询参数
#[derive(Debug, Deserialize)]
pub struct FuturesSpotPricePreviousQuery {
    pub date: String,  // 交易日期 YYYYMMDD
}


/// 期货现货价格日期范围查询参数
#[derive(Debug, Deserialize)]
pub struct FuturesSpotPriceDailyQuery {
    pub start_date: String,              // 开始日期 YYYYMMDD
    pub end_date: String,                // 结束日期 YYYYMMDD
    pub symbols: Option<String>,         // 品种代码列表，逗号分隔，如"RB,CU"，为空时返回所有品种
}


/// 期货持仓排名汇总数据
/// 对应 akshare 的 get_rank_sum() 返回结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RankSum {
    pub symbol: String,                      // 标的合约
    pub variety: String,                     // 商品品种
    pub vol_top5: i64,                       // 成交量前5会员成交量总和
    pub vol_chg_top5: i64,                   // 成交量前5会员成交量变化总和
    pub long_open_interest_top5: i64,        // 持多单前5会员持多单总和
    pub long_open_interest_chg_top5: i64,    // 持多单前5会员持多单变化总和
    pub short_open_interest_top5: i64,       // 持空单前5会员持空单总和
    pub short_open_interest_chg_top5: i64,   // 持空单前5会员持空单变化总和
    pub vol_top10: i64,                      // 成交量前10会员成交量总和
    pub vol_chg_top10: i64,                  // 成交量前10会员成交量变化总和
    pub long_open_interest_top10: i64,       // 持多单前10会员持多单总和
    pub long_open_interest_chg_top10: i64,   // 持多单前10会员持多单变化总和
    pub short_open_interest_top10: i64,      // 持空单前10会员持空单总和
    pub short_open_interest_chg_top10: i64,  // 持空单前10会员持空单变化总和
    pub vol_top15: i64,                      // 成交量前15会员成交量总和
    pub vol_chg_top15: i64,                  // 成交量前15会员成交量变化总和
    pub long_open_interest_top15: i64,       // 持多单前15会员持多单总和
    pub long_open_interest_chg_top15: i64,   // 持多单前15会员持多单变化总和
    pub short_open_interest_top15: i64,      // 持空单前15会员持空单总和
    pub short_open_interest_chg_top15: i64,  // 持空单前15会员持空单变化总和
    pub vol_top20: i64,                      // 成交量前20会员成交量总和
    pub vol_chg_top20: i64,                  // 成交量前20会员成交量变化总和
    pub long_open_interest_top20: i64,       // 持多单前20会员持多单总和
    pub long_open_interest_chg_top20: i64,   // 持多单前20会员持多单变化总和
    pub short_open_interest_top20: i64,      // 持空单前20会员持空单总和
    pub short_open_interest_chg_top20: i64,  // 持空单前20会员持空单变化总和
    pub date: String,                        // 日期 YYYYMMDD
}

/// 期货持仓排名原始数据（单个会员）
#[derive(Debug, Clone)]
pub struct PositionRankRow {
    pub rank: i32,                           // 排名
    pub vol_party_name: String,              // 成交量排序的当前名次会员
    pub vol: i64,                            // 该会员成交量
    pub vol_chg: i64,                        // 该会员成交量变化量
    pub long_party_name: String,             // 持多单排序的当前名次会员
    pub long_open_interest: i64,             // 该会员持多单
    pub long_open_interest_chg: i64,         // 该会员持多单变化量
    pub short_party_name: String,            // 持空单排序的当前名次会员
    pub short_open_interest: i64,            // 该会员持空单
    pub short_open_interest_chg: i64,        // 该会员持空单变化量
    pub symbol: String,                      // 标的合约
    pub variety: String,                     // 品种
}

/// 期货持仓排名日线查询参数
#[derive(Debug, Deserialize)]
pub struct RankSumDailyQuery {
    pub start_date: String,              // 开始日期 YYYYMMDD
    pub end_date: String,                // 结束日期 YYYYMMDD
    pub vars: Option<String>,            // 品种代码列表，逗号分隔，如"RB,CU"，为空时返回所有品种
}

/// 期货持仓排名表数据（单个会员）
/// 对应 akshare 的 get_shfe_rank_table/get_dce_rank_table/get_cffex_rank_table/get_rank_table_czce 返回结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PositionRankData {
    pub rank: i32,                           // 排名
    pub vol_party_name: String,              // 成交量排序的当前名次会员
    pub vol: i64,                            // 该会员成交量
    pub vol_chg: i64,                        // 该会员成交量变化量
    pub long_party_name: String,             // 持多单排序的当前名次会员
    pub long_open_interest: i64,             // 该会员持多单
    pub long_open_interest_chg: i64,         // 该会员持多单变化量
    pub short_party_name: String,            // 持空单排序的当前名次会员
    pub short_open_interest: i64,            // 该会员持空单
    pub short_open_interest_chg: i64,        // 该会员持空单变化量
    pub symbol: String,                      // 标的合约
    pub variety: String,                     // 品种
}

/// 期货持仓排名表查询参数
#[derive(Debug, Deserialize)]
pub struct RankTableQuery {
    pub date: String,                        // 交易日期 YYYYMMDD
    pub vars: Option<String>,                // 品种代码列表，逗号分隔，如"RB,CU"，为空时返回所有品种
}

/// 期货持仓排名表响应（按合约分组）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RankTableResponse {
    pub symbol: String,                      // 合约代码
    pub data: Vec<PositionRankData>,         // 排名数据列表
}


/// 郑商所仓单日报数据
/// 对应 akshare 的 futures_warehouse_receipt_czce() 返回结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CzceWarehouseReceipt {
    pub warehouse: String,                   // 仓库简称
    pub warehouse_receipt: Option<i64>,      // 仓单数量
    pub valid_forecast: Option<i64>,         // 有效预报
    pub change: Option<i64>,                 // 增减
}

/// 郑商所仓单日报响应（按品种分组）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CzceWarehouseReceiptResponse {
    pub symbol: String,                      // 品种代码
    pub data: Vec<CzceWarehouseReceipt>,     // 仓单数据列表
}


/// 大商所仓单日报数据
/// 对应 akshare 的 futures_warehouse_receipt_dce() 返回结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DceWarehouseReceipt {
    pub variety_code: String,                // 品种代码
    pub variety_name: String,                // 品种名称
    pub warehouse: String,                   // 仓库/分库
    pub delivery_location: Option<String>,   // 可选提货地点/分库-数量
    pub last_receipt: i64,                   // 昨日仓单量（手）
    pub today_receipt: i64,                  // 今日仓单量（手）
    pub change: i64,                         // 增减（手）
}


/// 上期所仓单日报数据
/// 对应 akshare 的 futures_shfe_warehouse_receipt() 返回结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShfeWarehouseReceipt {
    pub variety: String,                     // 品种名称
    pub region: String,                      // 地区名称
    pub warehouse: String,                   // 仓库简称
    pub last_receipt: i64,                   // 昨日仓单量
    pub today_receipt: i64,                  // 今日仓单量
    pub change: i64,                         // 仓单增减
    pub unit: String,                        // 单位
}

/// 上期所仓单日报响应（按品种分组）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShfeWarehouseReceiptResponse {
    pub symbol: String,                      // 品种代码
    pub data: Vec<ShfeWarehouseReceipt>,     // 仓单数据列表
}


/// 广期所仓单日报数据
/// 对应 akshare 的 futures_gfex_warehouse_receipt() 返回结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GfexWarehouseReceipt {
    pub variety: String,                     // 品种名称
    pub warehouse: String,                   // 仓库/分库
    pub last_receipt: i64,                   // 昨日仓单量
    pub today_receipt: i64,                  // 今日仓单量
    pub change: i64,                         // 增减
}

/// 广期所仓单日报响应（按品种分组）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GfexWarehouseReceiptResponse {
    pub symbol: String,                      // 品种代码
    pub data: Vec<GfexWarehouseReceipt>,     // 仓单数据列表
}


/// 新浪期货持仓排名数据
/// 对应 akshare 的 futures_hold_pos_sina() 返回结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SinaHoldPosition {
    pub rank: i32,                           // 名次
    pub company: String,                     // 期货公司
    pub value: i64,                          // 数值（成交量/多单持仓/空单持仓）
    pub change: i64,                         // 比上交易日增减
}

/// 新浪期货持仓类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SinaHoldPosType {
    Volume,     // 成交量
    Long,       // 多单持仓
    Short,      // 空单持仓
}

impl SinaHoldPosType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "成交量" | "volume" | "vol" => Some(Self::Volume),
            "多单持仓" | "多单" | "long" => Some(Self::Long),
            "空单持仓" | "空单" | "short" => Some(Self::Short),
            _ => None,
        }
    }
    
    pub fn table_index(&self) -> usize {
        match self {
            Self::Volume => 2,
            Self::Long => 3,
            Self::Short => 4,
        }
    }
    
    pub fn value_column_name(&self) -> &'static str {
        match self {
            Self::Volume => "成交量",
            Self::Long => "多单持仓",
            Self::Short => "空单持仓",
        }
    }
}
