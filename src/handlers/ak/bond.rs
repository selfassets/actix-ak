//! 债券数据 HTTP 处理器
//!
//! 提供可转债、中国/美国国债收益率等数据端点

use crate::models::{ak::bond::BondCbondQuery, ak::bond::BondQuery, ApiResponse};
use crate::services::ak::bond;
use actix_web::{web, HttpResponse, Result};

/// 获取沪深可转债实时行情
///
/// GET /api/v1/ak/bond/zh_cov_spot
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/bond/zh_cov_spot",
        tag = "bond",
        responses(
            (status = 200, description = "成功获取沪深可转债实时行情", body = ApiResponse<Vec<BondZhCovSpotItem>>)
        )
    )
)]
pub async fn get_bond_zh_cov_spot() -> Result<HttpResponse> {
    match bond::get_bond_zh_cov_spot().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取中国国债收益率历史数据（新浪源）
///
/// GET /api/v1/ak/bond/gb_zh_sina
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/bond/gb_zh_sina",
        tag = "bond",
        params(
            BondQuery
        ),
        responses(
            (status = 200, description = "成功获取中国国债收益率数据", body = ApiResponse<Vec<BondGbKlineItem>>)
        )
    )
)]
pub async fn get_bond_gb_zh_sina(query: web::Query<BondQuery>) -> Result<HttpResponse> {
    match bond::get_bond_gb_zh_sina(query.into_inner()).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取美国国债收益率历史数据（新浪源）
///
/// GET /api/v1/ak/bond/gb_us_sina
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/bond/gb_us_sina",
        tag = "bond",
        params(
            BondQuery
        ),
        responses(
            (status = 200, description = "成功获取美国国债收益率数据", body = ApiResponse<Vec<BondGbKlineItem>>)
        )
    )
)]
pub async fn get_bond_gb_us_sina(query: web::Query<BondQuery>) -> Result<HttpResponse> {
    match bond::get_bond_gb_us_sina(query.into_inner()).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取中美国债收益率对比数据（东方财富）
///
/// GET /api/v1/ak/bond/zh_us_rate
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/bond/zh_us_rate",
        tag = "bond",
        responses(
            (status = 200, description = "成功获取中美国债收益率数据", body = ApiResponse<Vec<BondZhUsRateItem>>)
        )
    )
)]
pub async fn get_bond_zh_us_rate() -> Result<HttpResponse> {
    match bond::get_bond_zh_us_rate().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取上证质押式国债逆回购行情（东方财富）
///
/// GET /api/v1/ak/bond/sh_buy_back
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/bond/sh_buy_back",
        tag = "bond",
        responses(
            (status = 200, description = "成功获取上证质押式国债逆回购行情", body = ApiResponse<Vec<BondBuyBackItem>>)
        )
    )
)]
pub async fn get_bond_sh_buy_back() -> Result<HttpResponse> {
    match bond::get_bond_sh_buy_back().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取深证质押式国债逆回购行情（东方财富）
///
/// GET /api/v1/ak/bond/sz_buy_back
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/bond/sz_buy_back",
        tag = "bond",
        responses(
            (status = 200, description = "成功获取深证质押式国债逆回购行情", body = ApiResponse<Vec<BondBuyBackItem>>)
        )
    )
)]
pub async fn get_bond_sz_buy_back() -> Result<HttpResponse> {
    match bond::get_bond_sz_buy_back().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取集思录可转债等权指数历史
///
/// GET /api/v1/ak/bond/cb_index_jsl
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/bond/cb_index_jsl",
        tag = "bond",
        responses(
            (status = 200, description = "成功获取集思录可转债等权指数", body = ApiResponse<Vec<BondJslItem>>)
        )
    )
)]
pub async fn get_bond_cb_index_jsl() -> Result<HttpResponse> {
    match bond::get_bond_cb_index_jsl().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取集思录可转债强赎信息列表
///
/// GET /api/v1/ak/bond/cb_redeem_jsl
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/bond/cb_redeem_jsl",
        tag = "bond",
        responses(
            (status = 200, description = "成功获取集思录可转债强赎列表", body = ApiResponse<Vec<BondJslItem>>)
        )
    )
)]
pub async fn get_bond_cb_redeem_jsl() -> Result<HttpResponse> {
    match bond::get_bond_cb_redeem_jsl().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取新浪财经可转债详情资料
///
/// GET /api/v1/ak/bond/cb_profile_sina
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/bond/cb_profile_sina",
        tag = "bond",
        params(
            BondQuery
        ),
        responses(
            (status = 200, description = "成功获取新浪可转债详情资料", body = ApiResponse<Vec<BondCbProfileItem>>)
        )
    )
)]
pub async fn get_bond_cb_profile_sina(query: web::Query<BondQuery>) -> Result<HttpResponse> {
    match bond::get_bond_cb_profile_sina(query.into_inner()).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取东方财富网可转债比价表数据
///
/// GET /api/v1/ak/bond/cov_comparison
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/bond/cov_comparison",
        tag = "bond",
        responses(
            (status = 200, description = "成功获取可转债比价表", body = ApiResponse<Vec<BondCovComparisonItem>>)
        )
    )
)]
pub async fn get_bond_cov_comparison() -> Result<HttpResponse> {
    match bond::get_bond_cov_comparison().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取新浪财经沪深债券实时行情数据
///
/// GET /api/v1/ak/bond/zh_hs_spot
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/bond/zh_hs_spot",
        tag = "bond",
        responses(
            (status = 200, description = "成功获取沪深债券实时行情", body = ApiResponse<Vec<BondZhHsSpotItem>>)
        )
    )
)]
pub async fn get_bond_zh_hs_spot() -> Result<HttpResponse> {
    match bond::get_bond_zh_hs_spot().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取质押式国债逆回购历史 K 线行情
///
/// GET /api/v1/ak/bond/buy_back_hist
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/bond/buy_back_hist",
        tag = "bond",
        params(
            BondQuery
        ),
        responses(
            (status = 200, description = "成功获取质押式国债逆回购历史 K 线行情", body = ApiResponse<Vec<BondGbKlineItem>>)
        )
    )
)]
pub async fn get_bond_buy_back_hist(query: web::Query<BondQuery>) -> Result<HttpResponse> {
    match bond::get_bond_buy_back_hist_em(query.into_inner()).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取中国外汇交易中心收益率曲线品种映射表
///
/// GET /api/v1/ak/bond/china_close_return_map
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/bond/china_close_return_map",
        tag = "bond",
        responses(
            (status = 200, description = "成功获取收益率曲线品种映射表", body = ApiResponse<Vec<BondChinaMoneyItem>>)
        )
    )
)]
pub async fn get_bond_china_close_return_map() -> Result<HttpResponse> {
    match bond::get_bond_china_close_return_map().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取上交所债券现货概览汇总数据
///
/// GET /api/v1/ak/bond/cash_summary_sse
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/bond/cash_summary_sse",
        tag = "bond",
        params(
            BondQuery
        ),
        responses(
            (status = 200, description = "成功获取上交所债券现货概览", body = ApiResponse<Vec<BondSseSummaryItem>>)
        )
    )
)]
pub async fn get_bond_cash_summary_sse(query: web::Query<BondQuery>) -> Result<HttpResponse> {
    match bond::get_bond_cash_summary_sse(query.into_inner()).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取上交所债券成交概览汇总数据
///
/// GET /api/v1/ak/bond/deal_summary_sse
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/bond/deal_summary_sse",
        tag = "bond",
        params(
            BondQuery
        ),
        responses(
            (status = 200, description = "成功获取上交所债券成交概览", body = ApiResponse<Vec<BondSseSummaryItem>>)
        )
    )
)]
pub async fn get_bond_deal_summary_sse(query: web::Query<BondQuery>) -> Result<HttpResponse> {
    match bond::get_bond_deal_summary_sse(query.into_inner()).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取同花顺可转债基本信息列表
///
/// GET /api/v1/ak/bond/zh_cov_info_ths
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/bond/zh_cov_info_ths",
        tag = "bond",
        responses(
            (status = 200, description = "成功获取同花顺可转债基本信息", body = ApiResponse<Vec<BondCovInfoThsItem>>)
        )
    )
)]
pub async fn get_bond_zh_cov_info_ths() -> Result<HttpResponse> {
    match bond::get_bond_zh_cov_info_ths().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取新浪财经可转债概况汇总
///
/// GET /api/v1/ak/bond/cb_summary_sina
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/bond/cb_summary_sina",
        tag = "bond",
        params(
            BondQuery
        ),
        responses(
            (status = 200, description = "成功获取新浪可转债概况汇总", body = ApiResponse<Vec<BondCbProfileItem>>)
        )
    )
)]
pub async fn get_bond_cb_summary_sina(query: web::Query<BondQuery>) -> Result<HttpResponse> {
    match bond::get_bond_cb_summary_sina(query.into_inner()).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取中国外汇交易中心现券市场做市报价
///
/// GET /api/v1/ak/bond/spot_quote
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/bond/spot_quote",
        tag = "bond",
        responses(
            (status = 200, description = "成功获取现券做市报价", body = ApiResponse<Vec<BondSpotQuoteItem>>)
        )
    )
)]
pub async fn get_bond_spot_quote() -> Result<HttpResponse> {
    match bond::get_bond_spot_quote().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取中国外汇交易中心现券市场成交行情
///
/// GET /api/v1/ak/bond/spot_deal
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/bond/spot_deal",
        tag = "bond",
        responses(
            (status = 200, description = "成功获取现券成交行情", body = ApiResponse<Vec<BondSpotDealItem>>)
        )
    )
)]
pub async fn get_bond_spot_deal() -> Result<HttpResponse> {
    match bond::get_bond_spot_deal().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取中国债券信息网国债及各期限收益率曲线
///
/// GET /api/v1/ak/bond/china_yield
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/bond/china_yield",
        tag = "bond",
        params(
            BondQuery
        ),
        responses(
            (status = 200, description = "成功获取中国债券信息网国债收益率曲线", body = ApiResponse<Vec<BondChinaYieldItem>>)
        )
    )
)]
pub async fn get_bond_china_yield(query: web::Query<BondQuery>) -> Result<HttpResponse> {
    match bond::get_bond_china_yield(query.into_inner()).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取中国货币网债券信息查询参数 (主承销商/债券类型/息票类型/评级等)
///
/// GET /api/v1/ak/bond/info_cm_query
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/bond/info_cm_query",
        tag = "bond",
        params(
            BondQuery
        ),
        responses(
            (status = 200, description = "成功获取中国货币网债券查询参数", body = ApiResponse<Vec<BondInfoCmQueryItem>>)
        )
    )
)]
pub async fn get_bond_info_cm_query(query: web::Query<BondQuery>) -> Result<HttpResponse> {
    match bond::get_bond_info_cm_query(query.into_inner()).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取中国货币网债券信息列表
///
/// GET /api/v1/ak/bond/info_cm
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/bond/info_cm",
        tag = "bond",
        params(
            BondQuery
        ),
        responses(
            (status = 200, description = "成功获取中国货币网债券信息列表", body = ApiResponse<Vec<BondInfoCmItem>>)
        )
    )
)]
pub async fn get_bond_info_cm(query: web::Query<BondQuery>) -> Result<HttpResponse> {
    match bond::get_bond_info_cm(query.into_inner()).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取中国银行间市场交易商协会 (NAFMII) 非金融企业债务融资工具注册信息
///
/// GET /api/v1/ak/bond/debt_nafmii
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/bond/debt_nafmii",
        tag = "bond",
        params(
            BondQuery
        ),
        responses(
            (status = 200, description = "成功获取 NAFMII 债务融资工具注册信息", body = ApiResponse<Vec<BondDebtNafmiiItem>>)
        )
    )
)]
pub async fn get_bond_debt_nafmii(query: web::Query<BondQuery>) -> Result<HttpResponse> {
    match bond::get_bond_debt_nafmii(query.into_inner()).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取中国货币网单只债券详情信息
///
/// GET /api/v1/ak/bond/info_detail_cm
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/bond/info_detail_cm",
        tag = "bond",
        params(
            BondQuery
        ),
        responses(
            (status = 200, description = "成功获取中国货币网单只债券详情", body = ApiResponse<BondInfoDetailCmItem>)
        )
    )
)]
pub async fn get_bond_info_detail_cm(query: web::Query<BondQuery>) -> Result<HttpResponse> {
    match bond::get_bond_info_detail_cm(query.into_inner()).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取巨潮资讯债券发行数据
///
/// GET /api/v1/ak/bond/issue_cninfo
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/bond/issue_cninfo",
        tag = "bond",
        params(
            BondQuery
        ),
        responses(
            (status = 200, description = "成功获取巨潮资讯债券发行数据", body = ApiResponse<Vec<BondIssueCninfoItem>>)
        )
    )
)]
pub async fn get_bond_issue_cninfo(query: web::Query<BondQuery>) -> Result<HttpResponse> {
    match bond::get_bond_issue_cninfo(query.into_inner()).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取东方财富可转债价值分析数据 (溢价率分析)
///
/// GET /api/v1/ak/bond/zh_cov_value_analysis
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/bond/zh_cov_value_analysis",
        tag = "bond",
        params(
            BondQuery
        ),
        responses(
            (status = 200, description = "成功获取可转债价值分析", body = ApiResponse<Vec<BondZhCovValueAnalysisItem>>)
        )
    )
)]
pub async fn get_bond_zh_cov_value_analysis(query: web::Query<BondQuery>) -> Result<HttpResponse> {
    match bond::get_bond_zh_cov_value_analysis(query.into_inner()).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取中国债券信息网中债国债指数
///
/// GET /api/v1/ak/bond/treasury_index_cbond
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/bond/treasury_index_cbond",
        tag = "bond",
        params(
            BondQuery
        ),
        responses(
            (status = 200, description = "成功获取中债国债指数", body = ApiResponse<Vec<BondCbondIndexItem>>)
        )
    )
)]
pub async fn get_bond_treasury_index_cbond(query: web::Query<BondQuery>) -> Result<HttpResponse> {
    match bond::get_bond_treasury_index_cbond(query.into_inner()).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取沪深可转债历史日 K 线行情
///
/// GET /api/v1/ak/bond/zh_hs_cov_daily
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/bond/zh_hs_cov_daily",
        tag = "bond",
        params(
            BondQuery
        ),
        responses(
            (status = 200, description = "成功获取沪深可转债历史日 K 线行情", body = ApiResponse<Vec<BondGbKlineItem>>)
        )
    )
)]
pub async fn get_bond_zh_hs_cov_daily(query: web::Query<BondQuery>) -> Result<HttpResponse> {
    match bond::get_bond_zh_hs_cov_daily(query.into_inner()).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取沪深现券/债券历史日 K 线行情
///
/// GET /api/v1/ak/bond/zh_hs_daily
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/bond/zh_hs_daily",
        tag = "bond",
        params(
            BondQuery
        ),
        responses(
            (status = 200, description = "成功获取沪深现券历史日 K 线行情", body = ApiResponse<Vec<BondGbKlineItem>>)
        )
    )
)]
pub async fn get_bond_zh_hs_daily(query: web::Query<BondQuery>) -> Result<HttpResponse> {
    match bond::get_bond_zh_hs_daily(query.into_inner()).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取集思录可转债转股价调整日志
///
/// GET /api/v1/ak/bond/cb_adj_logs_jsl
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/bond/cb_adj_logs_jsl",
        tag = "bond",
        params(
            BondQuery
        ),
        responses(
            (status = 200, description = "成功获取集思录转股价调整日志", body = ApiResponse<Vec<BondCbAdjLogJslItem>>)
        )
    )
)]
pub async fn get_bond_cb_adj_logs_jsl(query: web::Query<BondQuery>) -> Result<HttpResponse> {
    match bond::get_bond_cb_adj_logs_jsl(query.into_inner()).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取中债指数可选项列表
///
/// GET /api/v1/ak/bond/available_index_cbond
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/bond/available_index_cbond",
        tag = "bond",
        responses(
            (status = 200, description = "成功获取中债指数可选项列表", body = ApiResponse<Vec<BondAvailableIndexItem>>)
        )
    )
)]
pub async fn get_bond_available_index_cbond() -> Result<HttpResponse> {
    match bond::get_bond_available_index_cbond().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取中国外汇交易中心收盘收益率曲线历史数据
///
/// GET /api/v1/ak/bond/china_close_return
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/bond/china_close_return",
        tag = "bond",
        params(
            BondQuery
        ),
        responses(
            (status = 200, description = "成功获取中国外汇交易中心收盘收益率曲线", body = ApiResponse<Vec<BondChinaCloseReturnItem>>)
        )
    )
)]
pub async fn get_bond_china_close_return(query: web::Query<BondQuery>) -> Result<HttpResponse> {
    match bond::get_bond_china_close_return(query.into_inner()).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取中国债券信息网中债通用指数序列
///
/// GET /api/v1/ak/bond/index_general_cbond
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/bond/index_general_cbond",
        tag = "bond",
        params(
            BondCbondQuery
        ),
        responses(
            (status = 200, description = "成功获取中债通用指数序列", body = ApiResponse<Vec<BondCbondIndexItem>>)
        )
    )
)]
pub async fn get_bond_index_general_cbond(
    query: web::Query<BondCbondQuery>,
) -> Result<HttpResponse> {
    match bond::get_bond_index_general_cbond(query.into_inner()).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 配置债券路由
///
/// 挂载路径：/bond
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/bond")
            .route("/zh_cov_spot", web::get().to(get_bond_zh_cov_spot))
            .route("/gb_zh_sina", web::get().to(get_bond_gb_zh_sina))
            .route("/gb_us_sina", web::get().to(get_bond_gb_us_sina))
            .route("/zh_us_rate", web::get().to(get_bond_zh_us_rate))
            .route("/sh_buy_back", web::get().to(get_bond_sh_buy_back))
            .route("/sz_buy_back", web::get().to(get_bond_sz_buy_back))
            .route("/buy_back_hist", web::get().to(get_bond_buy_back_hist))
            .route("/cb_index_jsl", web::get().to(get_bond_cb_index_jsl))
            .route("/cb_redeem_jsl", web::get().to(get_bond_cb_redeem_jsl))
            .route("/cb_profile_sina", web::get().to(get_bond_cb_profile_sina))
            .route("/cov_comparison", web::get().to(get_bond_cov_comparison))
            .route("/zh_hs_spot", web::get().to(get_bond_zh_hs_spot))
            .route(
                "/china_close_return_map",
                web::get().to(get_bond_china_close_return_map),
            )
            .route(
                "/cash_summary_sse",
                web::get().to(get_bond_cash_summary_sse),
            )
            .route(
                "/deal_summary_sse",
                web::get().to(get_bond_deal_summary_sse),
            )
            .route("/zh_cov_info_ths", web::get().to(get_bond_zh_cov_info_ths))
            .route("/cb_summary_sina", web::get().to(get_bond_cb_summary_sina))
            .route("/spot_quote", web::get().to(get_bond_spot_quote))
            .route("/spot_deal", web::get().to(get_bond_spot_deal))
            .route("/china_yield", web::get().to(get_bond_china_yield))
            .route("/info_cm_query", web::get().to(get_bond_info_cm_query))
            .route("/info_cm", web::get().to(get_bond_info_cm))
            .route("/debt_nafmii", web::get().to(get_bond_debt_nafmii))
            .route("/info_detail_cm", web::get().to(get_bond_info_detail_cm))
            .route("/issue_cninfo", web::get().to(get_bond_issue_cninfo))
            .route(
                "/zh_cov_value_analysis",
                web::get().to(get_bond_zh_cov_value_analysis),
            )
            .route(
                "/treasury_index_cbond",
                web::get().to(get_bond_treasury_index_cbond),
            )
            .route("/zh_hs_cov_daily", web::get().to(get_bond_zh_hs_cov_daily))
            .route("/zh_hs_daily", web::get().to(get_bond_zh_hs_daily))
            .route("/cb_adj_logs_jsl", web::get().to(get_bond_cb_adj_logs_jsl))
            .route(
                "/available_index_cbond",
                web::get().to(get_bond_available_index_cbond),
            )
            .route(
                "/index_general_cbond",
                web::get().to(get_bond_index_general_cbond),
            )
            .route(
                "/china_close_return",
                web::get().to(get_bond_china_close_return),
            ),
    );
}
