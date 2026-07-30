//! 银行与金融监管数据模型
//!
//! 定义行政处罚与银保监会/金融监管总局数据结构

use serde::{Deserialize, Serialize};

/// 行政处罚查询参数
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::IntoParams, utoipa::ToSchema))]
pub struct BankFjcfQuery {
    /// 级别/分类：选择范围 {"机关", "本级", "分局本级"}，默认 "分局本级"
    pub item: Option<String>,
    /// 起始页码，默认 1
    pub begin: Option<i32>,
    /// 获取的页数，默认 1
    pub page: Option<i32>,
}

/// 行政处罚列表条目
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct BankFjcfListItem {
    /// 处罚文档 ID
    pub doc_id: Option<String>,
    /// 文档副标题/文号
    pub doc_subtitle: Option<String>,
    /// 发布日期
    pub publish_date: Option<String>,
    /// 文档文件 URL
    pub doc_file_url: Option<String>,
    /// 文档标题
    pub doc_title: Option<String>,
    /// 类型
    pub general_type: Option<serde_json::Value>,
}

/// 行政处罚公开表详细数据
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct BankFjcfDetailItem {
    /// 行政处罚决定书文号
    pub doc_number: Option<String>,
    /// 被处罚人姓名
    pub name: Option<String>,
    /// 单位
    pub unit: Option<String>,
    /// 单位名称
    pub company_name: Option<String>,
    /// 主要负责人姓名
    pub principal_name: Option<String>,
    /// 主要违法违规事实（案由）
    pub main_facts: Option<String>,
    /// 行政处罚依据
    pub penalty_basis: Option<String>,
    /// 行政处罚决定
    pub penalty_decision: Option<String>,
    /// 作出处罚决定的机关名称
    pub agency_name: Option<String>,
    /// 作出处罚决定的日期
    pub decision_date: Option<String>,
    /// 处罚ID
    pub penalty_id: Option<String>,
    /// 处罚公布日期
    pub publish_date: Option<String>,
}
