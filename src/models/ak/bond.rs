//! 债券数据模型
//!
//! 定义可转债、国债收益率、债券回购等请求与响应数据结构

use serde::{Deserialize, Serialize};

/// 债券查询参数
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::IntoParams, utoipa::ToSchema))]
pub struct BondQuery {
    /// 债券代码或分类标志，例如 "113527"、"中国10年期国债" 或 "204001"
    pub symbol: Option<String>,
    /// 开始日期，格式 YYYYMMDD
    pub start_date: Option<String>,
    /// 结束日期，格式 YYYYMMDD
    pub end_date: Option<String>,
}

/// 沪深可转债实时行情条目
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct BondZhCovSpotItem {
    /// 可转债代码
    pub code: Option<String>,
    /// 可转债简称
    pub name: Option<String>,
    /// 现价
    pub trade: Option<f64>,
    /// 涨跌额
    pub change_price: Option<f64>,
    /// 涨跌幅 %
    pub change_percent: Option<f64>,
    /// 动态包含全部响应字段
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// 国债收益率 K 线 / 历史行情数据条目
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct BondGbKlineItem {
    /// 日期
    pub date: Option<String>,
    /// 开盘价/收益率
    pub open: Option<f64>,
    /// 最高价/收益率
    pub high: Option<f64>,
    /// 最低价/收益率
    pub low: Option<f64>,
    /// 收盘价/收益率
    pub close: Option<f64>,
    /// 成交量
    pub volume: Option<f64>,
}

/// 中美国债收益率对比数据条目
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct BondZhUsRateItem {
    /// 日期
    pub date: Option<String>,
    /// 中国1年期国债收益率
    pub cn_1y: Option<f64>,
    /// 中国10年期国债收益率
    pub cn_10y: Option<f64>,
    /// 美国1年期国债收益率
    pub us_1y: Option<f64>,
    /// 美国10年期国债收益率
    pub us_10y: Option<f64>,
    /// 10年期中美利差
    pub spread_10y: Option<f64>,
    /// 其它详细字段
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// 可转债比价表数据条目
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct BondCovComparisonItem {
    /// 动态包含东方财富可转债比价表的所有字段映射
    #[serde(flatten)]
    pub data: std::collections::HashMap<String, serde_json::Value>,
}

/// 集思录可转债转股价调整日志条目
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct BondCbAdjLogJslItem {
    /// 股东大会/公告日期
    pub date: Option<String>,
    /// 下修前转股价
    pub before_price: Option<f64>,
    /// 下修后转股价
    pub after_price: Option<f64>,
    /// 调整生效日期
    pub effective_date: Option<String>,
    /// 备注/说明
    pub remark: Option<String>,
}

/// 集思录可转债指数与强赎等通用数据条目
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct BondJslItem {
    /// 动态包含集思录 JSON 的所有字段映射
    #[serde(flatten)]
    pub data: std::collections::HashMap<String, serde_json::Value>,
}

/// 可转债详情/基本信息项条目
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct BondCbProfileItem {
    /// 项目/指标名称
    pub item: String,
    /// 对应数值/描述
    pub value: String,
}

/// 质押式债券回购行情数据条目
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct BondBuyBackItem {
    /// 代码
    pub code: Option<String>,
    /// 名称
    pub name: Option<String>,
    /// 最新价（收益率 %）
    pub price: Option<f64>,
    /// 涨跌额
    pub change_price: Option<f64>,
    /// 涨跌幅 %
    pub change_percent: Option<f64>,
    /// 今开
    pub open: Option<f64>,
    /// 最高
    pub high: Option<f64>,
    /// 最低
    pub low: Option<f64>,
    /// 昨收
    pub close: Option<f64>,
    /// 成交量
    pub volume: Option<f64>,
    /// 成交额
    pub amount: Option<f64>,
}

/// 沪深债券实时行情数据条目
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct BondZhHsSpotItem {
    /// 债券代码
    pub symbol: Option<String>,
    /// 债券名称
    pub name: Option<String>,
    /// 最新价
    pub trade: Option<f64>,
    /// 涨跌额
    pub change_price: Option<f64>,
    /// 涨跌幅 %
    pub change_percent: Option<f64>,
    /// 买入价
    pub buy: Option<f64>,
    /// 卖出价
    pub sell: Option<f64>,
    /// 昨收
    pub settlement: Option<f64>,
    /// 今开
    pub open: Option<f64>,
    /// 最高
    pub high: Option<f64>,
    /// 最低
    pub low: Option<f64>,
    /// 成交量
    pub volume: Option<f64>,
    /// 成交额
    pub amount: Option<f64>,
}

/// 上交所债券现货及成交汇总条目
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct BondSseSummaryItem {
    /// 类型/名称
    pub name: Option<String>,
    /// 当日/托管统计值
    pub day_val: Option<f64>,
    /// 当年/市值等统计值
    pub year_val: Option<f64>,
    /// 面值等
    pub par_val: Option<f64>,
    /// 数据日期
    pub date: Option<String>,
}

/// 中国外汇交易中心(CFETS) 收益率曲线映射及债券通用条目
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct BondChinaMoneyItem {
    /// 动态包含响应字段
    #[serde(flatten)]
    pub data: std::collections::HashMap<String, serde_json::Value>,
}

/// 中国外汇交易中心 (ChinaMoney) 收盘收益率曲线条目
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct BondChinaCloseReturnItem {
    /// 日期
    pub date: Option<String>,
    /// 期限
    pub term: Option<f64>,
    /// 到期收益率 %
    pub ytm: Option<f64>,
    /// 即期收益率 %
    pub spot_rate: Option<f64>,
    /// 远期收益率 %
    pub forward_rate: Option<f64>,
}

/// 同花顺可转债信息条目
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct BondCovInfoThsItem {
    /// 动态包含同花顺可转债响应的所有字段
    #[serde(flatten)]
    pub data: std::collections::HashMap<String, serde_json::Value>,
}

/// 中国外汇交易中心(CFETS) 现券市场做市报价条目
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct BondSpotQuoteItem {
    /// 报价机构
    pub institution: Option<String>,
    /// 债券简称
    pub bond_name: Option<String>,
    /// 买入净价
    pub buy_clean_price: Option<f64>,
    /// 卖出净价
    pub sell_clean_price: Option<f64>,
    /// 买入收益率 %
    pub buy_yield: Option<f64>,
    /// 卖出收益率 %
    pub sell_yield: Option<f64>,
}

/// 中国外汇交易中心(CFETS) 现券市场成交行情条目
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct BondSpotDealItem {
    /// 债券简称
    pub bond_name: Option<String>,
    /// 涨跌
    pub change: Option<f64>,
    /// 加权收益率
    pub weighted_yield: Option<f64>,
    /// 成交净价
    pub clean_price: Option<f64>,
    /// 最新收益率
    pub latest_yield: Option<f64>,
    /// 交易量
    pub volume: Option<f64>,
}

/// 中国债券信息网(ChinaBond) 国债及各期限收益率条目
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct BondChinaYieldItem {
    /// 动态包含日期及各期限 (3月, 1年, 10年等) 收益率
    #[serde(flatten)]
    pub data: std::collections::HashMap<String, serde_json::Value>,
}

/// 中国货币网 (ChinaMoney) 债券信息查询条目
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct BondInfoCmItem {
    /// 动态包含中国货币网债券信息返回的各种列字段
    #[serde(flatten)]
    pub data: std::collections::HashMap<String, serde_json::Value>,
}

/// 中国货币网 (ChinaMoney) 债券参数查询条目
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct BondInfoCmQueryItem {
    /// 代码
    pub code: Option<String>,
    /// 名称/描述
    pub name: Option<String>,
}

/// 中国银行间市场交易商协会 (NAFMII) 债务融资工具注册条目
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct BondDebtNafmiiItem {
    /// 动态包含交易商协会返回的注册通知书文号、债券名称、金额、更新日期等
    #[serde(flatten)]
    pub data: std::collections::HashMap<String, serde_json::Value>,
}

/// 巨潮资讯 (Cninfo) 债券发行数据条目
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct BondIssueCninfoItem {
    /// 动态包含巨潮资讯债券发行的各类响应字段
    #[serde(flatten)]
    pub data: std::collections::HashMap<String, serde_json::Value>,
}

/// 中债指数通用序列数据条目
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct BondCbondIndexItem {
    /// 日期
    pub date: Option<String>,
    /// 指数值
    pub value: Option<f64>,
}

/// 中债指数可选项条目
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct BondAvailableIndexItem {
    /// 序号
    pub index: usize,
    /// 指数名称
    pub name: String,
}

/// 中债指数通用查询参数
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::IntoParams, utoipa::ToSchema))]
pub struct BondCbondQuery {
    /// 指数分类（例如 "新综合指数", "中债-国债指数"，默认 "新综合指数"）
    pub index_category: Option<String>,
    /// 指标分类（例如 "全价", "净价", "财富"，默认 "全价"）
    pub indicator: Option<String>,
    /// 期限分段（例如 "总值", "1年以下", "1-3年"，默认 "总值"）
    pub period: Option<String>,
}

/// 东方财富可转债价值分析 (溢价率分析) 条目
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct BondZhCovValueAnalysisItem {
    /// 动态包含可转债价值分析的各种响应字段 (日期, 转股价值, 纯债价值, 转股溢价率等)
    #[serde(flatten)]
    pub data: std::collections::HashMap<String, serde_json::Value>,
}

/// 中国货币网 (ChinaMoney) 单只债券详情数据模型
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct BondInfoDetailCmItem {
    /// 动态包含 ChinaMoney 债券详情中的各种基础字段
    #[serde(flatten)]
    pub data: std::collections::HashMap<String, serde_json::Value>,
}
