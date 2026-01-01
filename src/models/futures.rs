use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FuturesInfo {
    pub symbol: String,
    pub name: String,
    pub current_price: f64,
    pub change: f64,
    pub change_percent: f64,
    pub volume: u64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub settlement: Option<f64>,
    pub prev_settlement: Option<f64>,
    pub open_interest: Option<u64>,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FuturesHistoryData {
    pub symbol: String,
    pub date: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u64,
    pub settlement: Option<f64>,
    pub open_interest: Option<u64>,
}

/// 期货查询参数
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct FuturesQuery {
    pub symbol: Option<String>,
    pub exchange: Option<String>, // 交易所：DCE, CZCE, SHFE, INE, CFFEX, GFEX
    pub category: Option<String>, // 品种分类
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub limit: Option<usize>,
}

/// 交易所信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FuturesExchange {
    pub code: String,
    pub name: String,
    pub description: String,
}

/// 期货品种映射信息
/// 对应 akshare 的 futures_symbol_mark() 返回结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FuturesSymbolMark {
    pub exchange: String,      // 交易所名称（中文）
    pub symbol: String,        // 品种名称（如 PTA、铜）
    pub mark: String,          // 新浪API的node参数（如 pta_qh、tong_qh）
}

/// 期货合约详情
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FuturesContractDetail {
    pub symbol: String,              // 合约代码
    pub name: String,                // 合约名称
    pub exchange: String,            // 上市交易所
    pub trading_unit: String,        // 交易单位
    pub quote_unit: String,          // 报价单位
    pub min_price_change: String,    // 最小变动价位
    pub price_limit: String,         // 涨跌停板幅度
    pub contract_months: String,     // 合约交割月份
    pub trading_hours: String,       // 交易时间
    pub last_trading_day: String,    // 最后交易日
    pub last_delivery_day: String,   // 最后交割日
    pub delivery_grade: String,      // 交割品级
    pub margin: String,              // 最低交易保证金
    pub delivery_method: String,     // 交割方式
}

/// 外盘期货品种信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ForeignFuturesSymbol {
    pub symbol: String,   // 品种中文名
    pub code: String,     // 品种代码
}

/// 主力连续合约信息
/// 对应 akshare 的 futures_display_main_sina() 返回结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FuturesMainContract {
    pub symbol: String,   // 合约代码（如 V0, RB0）
    pub name: String,     // 合约名称（如 PVC连续）
    pub exchange: String, // 交易所代码
}

/// 主力连续合约日K线数据
/// 对应 akshare 的 futures_main_sina() 返回结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FuturesMainDailyData {
    pub date: String,           // 日期
    pub open: f64,              // 开盘价
    pub high: f64,              // 最高价
    pub low: f64,               // 最低价
    pub close: f64,             // 收盘价
    pub volume: u64,            // 成交量
    pub hold: u64,              // 持仓量
    pub settle: Option<f64>,    // 动态结算价
}

/// 期货持仓排名数据
/// 对应 akshare 的 futures_hold_pos_sina() 返回结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FuturesHoldPosition {
    pub rank: u32,              // 名次
    pub company: String,        // 期货公司
    pub value: i64,             // 数值（成交量/多单持仓/空单持仓）
    pub change: i64,            // 比上交易日增减
}

/// 持仓排名查询参数
#[derive(Debug, Deserialize)]
pub struct FuturesHoldPosQuery {
    pub pos_type: Option<String>,  // 类型：volume(成交量), long(多单持仓), short(空单持仓)
    pub contract: String,          // 合约代码
    pub date: String,              // 查询日期 YYYYMMDD
}

/// 主力连续日数据查询参数
#[derive(Debug, Deserialize)]
pub struct FuturesMainQuery {
    pub start_date: Option<String>,  // 开始日期 YYYYMMDD
    pub end_date: Option<String>,    // 结束日期 YYYYMMDD
}

/// 外盘期货历史数据
/// 对应 akshare 的 futures_foreign_hist() 返回结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ForeignFuturesHistData {
    pub date: String,           // 日期
    pub open: f64,              // 开盘价
    pub high: f64,              // 最高价
    pub low: f64,               // 最低价
    pub close: f64,             // 收盘价
    pub volume: u64,            // 成交量
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
