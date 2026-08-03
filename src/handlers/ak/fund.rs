//! 基金市场(Fund) HTTP 处理器

use crate::models::ApiResponse;
use crate::services::ak::fund;
use actix_web::{web, HttpResponse, Result};

/// 获取天天基金网各公募基金名录
///
/// GET /api/v1/ak/fund/name_em
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/name_em",
        tag = "fund",
        responses(
            (status = 200, description = "成功获取基金名录数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_name_em() -> Result<HttpResponse> {
    match fund::fund_name_em().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取天天基金网基金申购状态和最新净值
///
/// GET /api/v1/ak/fund/purchase_em
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/purchase_em",
        tag = "fund",
        responses(
            (status = 200, description = "成功获取基金申购状态", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_purchase_em() -> Result<HttpResponse> {
    match fund::fund_purchase_em().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取东方财富全量 ETF 场内实时行情
///
/// GET /api/v1/ak/fund/etf_spot_em
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/etf_spot_em",
        tag = "fund",
        responses(
            (status = 200, description = "成功获取 ETF 实时行情", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_etf_spot_em() -> Result<HttpResponse> {
    match fund::fund_etf_spot_em().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取天天基金开放式基金业绩排行
///
/// GET /api/v1/ak/fund/open_fund_rank_em
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/open_fund_rank_em",
        tag = "fund",
        params(crate::models::ak::fund::FundQuery),
        responses(
            (status = 200, description = "成功获取开放式基金排行", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_open_fund_rank_em(
    query: web::Query<crate::models::ak::fund::FundQuery>,
) -> Result<HttpResponse> {
    let symbol = query.symbol.clone().unwrap_or_else(|| "all".to_string());
    match fund::fund_open_fund_rank_em(&symbol).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取天天基金全量基金经理列表
///
/// GET /api/v1/ak/fund/manager_em
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/manager_em",
        tag = "fund",
        responses(
            (status = 200, description = "成功获取基金经理列表", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_manager_em() -> Result<HttpResponse> {
    match fund::fund_manager_em().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取东方财富全量 LOF 实时行情
///
/// GET /api/v1/ak/fund/lof_spot_em
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/lof_spot_em",
        tag = "fund",
        responses(
            (status = 200, description = "成功获取 LOF 实时行情", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_lof_spot_em() -> Result<HttpResponse> {
    match fund::fund_lof_spot_em().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取天天基金单只基金历年持仓股票明细
///
/// GET /api/v1/ak/fund/portfolio_hold_em
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/portfolio_hold_em",
        tag = "fund",
        params(
            ("symbol" = Option<String>, Query, description = "基金代码，如 000001"),
            ("year" = Option<String>, Query, description = "年份，如 2023")
        ),
        responses(
            (status = 200, description = "成功获取基金持仓股票明细", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_portfolio_hold_em(
    query: web::Query<crate::models::ak::fortune::FortuneRankQuery>,
) -> Result<HttpResponse> {
    let symbol = query.symbol.clone().unwrap_or_else(|| "000001".to_string());
    let year = query.year.clone().unwrap_or_else(|| "2023".to_string());
    match fund::fund_portfolio_hold_em(&symbol, &year).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取天天基金全量基金评级数据
///
/// GET /api/v1/ak/fund/rating_all
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/rating_all",
        tag = "fund",
        responses(
            (status = 200, description = "成功获取全量基金评级数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_rating_all() -> Result<HttpResponse> {
    match fund::fund_rating_all().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取天天基金历史分红送配数据
///
/// GET /api/v1/ak/fund/fh_em
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/fh_em",
        tag = "fund",
        params(
            ("year" = Option<String>, Query, description = "年份，如 2023")
        ),
        responses(
            (status = 200, description = "成功获取历史分红送配数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_fh_em(
    query: web::Query<crate::models::ak::fortune::FortuneRankQuery>,
) -> Result<HttpResponse> {
    let year = query.year.clone().unwrap_or_else(|| "2023".to_string());
    match fund::fund_fh_em(&year).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取天天基金拆细折算明细数据
///
/// GET /api/v1/ak/fund/cf_em
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/cf_em",
        tag = "fund",
        params(
            ("year" = Option<String>, Query, description = "年份，如 2023")
        ),
        responses(
            (status = 200, description = "成功获取拆细折算数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_cf_em(
    query: web::Query<crate::models::ak::fortune::FortuneRankQuery>,
) -> Result<HttpResponse> {
    let year = query.year.clone().unwrap_or_else(|| "2023".to_string());
    match fund::fund_cf_em(&year).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取天天基金分红排行榜
///
/// GET /api/v1/ak/fund/fh_rank_em
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/fh_rank_em",
        tag = "fund",
        responses(
            (status = 200, description = "成功获取分红排行榜", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_fh_rank_em() -> Result<HttpResponse> {
    match fund::fund_fh_rank_em().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取新浪财经封闭式基金规模数据
///
/// GET /api/v1/ak/fund/scale_close_sina
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/scale_close_sina",
        tag = "fund",
        responses(
            (status = 200, description = "成功获取封闭式基金规模", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_scale_close_sina() -> Result<HttpResponse> {
    match fund::fund_scale_close_sina().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取基金公司管理规模排名
///
/// GET /api/v1/ak/fund/aum_em
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/aum_em",
        tag = "fund",
        responses(
            (status = 200, description = "成功获取基金公司管理规模排名", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_aum_em() -> Result<HttpResponse> {
    match fund::fund_aum_em().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取基金市场管理规模历史走势图
///
/// GET /api/v1/ak/fund/aum_trend_em
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/aum_trend_em",
        tag = "fund",
        responses(
            (status = 200, description = "成功获取管理规模历史走势图", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_aum_trend_em() -> Result<HttpResponse> {
    match fund::fund_aum_trend_em().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取天天基金盘中实时估算净值
///
/// GET /api/v1/ak/fund/value_estimation_em
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/value_estimation_em",
        tag = "fund",
        responses(
            (status = 200, description = "成功获取天天基金盘中实时估算", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_value_estimation_em() -> Result<HttpResponse> {
    match fund::fund_value_estimation_em().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取天天基金新成立基金全量数据
///
/// GET /api/v1/ak/fund/new_found_em
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/new_found_em",
        tag = "fund",
        responses(
            (status = 200, description = "成功获取新成立基金数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_new_found_em() -> Result<HttpResponse> {
    match fund::fund_new_found_em().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取天天基金规模变动历史明细
///
/// GET /api/v1/ak/fund/scale_change_em
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/scale_change_em",
        tag = "fund",
        responses(
            (status = 200, description = "成功获取基金规模变动数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_scale_change_em() -> Result<HttpResponse> {
    match fund::fund_scale_change_em().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取天天基金持有人机构与个人占比结构
///
/// GET /api/v1/ak/fund/hold_structure_em
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/hold_structure_em",
        tag = "fund",
        responses(
            (status = 200, description = "成功获取持有人结构数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_hold_structure_em() -> Result<HttpResponse> {
    match fund::fund_hold_structure_em().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取天天基金单只基金买卖费率与管理运作费用规则
///
/// GET /api/v1/ak/fund/fee_em
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/fee_em",
        tag = "fund",
        params(
            ("symbol" = Option<String>, Query, description = "基金代码，如 015641")
        ),
        responses(
            (status = 200, description = "成功获取基金费率与规则", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_fee_em(
    query: web::Query<crate::models::ak::fortune::FortuneRankQuery>,
) -> Result<HttpResponse> {
    let symbol = query.symbol.clone().unwrap_or_else(|| "015641".to_string());
    match fund::fund_fee_em(&symbol).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取东方财富 ETF 历史日线及分钟 K 线行情
///
/// GET /api/v1/ak/fund/etf_hist_em
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/etf_hist_em",
        tag = "fund",
        params(
            ("symbol" = Option<String>, Query, description = "ETF代码，如 510300"),
            ("indicator" = Option<String>, Query, description = "K线周期，如 daily(101), 1, 5, 15, 30, 60")
        ),
        responses(
            (status = 200, description = "成功获取 ETF 历史 K 线", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_etf_hist_em(
    query: web::Query<crate::models::ak::fortune::FortuneRankQuery>,
) -> Result<HttpResponse> {
    let symbol = query.symbol.clone().unwrap_or_else(|| "510300".to_string());
    let period = query
        .indicator
        .clone()
        .unwrap_or_else(|| "daily".to_string());
    match fund::fund_etf_hist_em(&symbol, &period).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取东方财富 LOF 历史日线 K 线行情
///
/// GET /api/v1/ak/fund/lof_hist_em
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/lof_hist_em",
        tag = "fund",
        params(
            ("symbol" = Option<String>, Query, description = "LOF代码，如 166009")
        ),
        responses(
            (status = 200, description = "成功获取 LOF 历史 K 线", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_lof_hist_em(
    query: web::Query<crate::models::ak::fortune::FortuneRankQuery>,
) -> Result<HttpResponse> {
    let symbol = query.symbol.clone().unwrap_or_else(|| "166009".to_string());
    match fund::fund_lof_hist_em(&symbol).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取天天基金全量开放式基金每日净值
///
/// GET /api/v1/ak/fund/open_fund_daily_em
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/open_fund_daily_em",
        tag = "fund",
        responses(
            (status = 200, description = "成功获取开放式基金每日净值", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_open_fund_daily_em() -> Result<HttpResponse> {
    match fund::fund_open_fund_daily_em().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取天天基金全量货币型基金每日收益与年化率
///
/// GET /api/v1/ak/fund/money_fund_daily_em
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/money_fund_daily_em",
        tag = "fund",
        responses(
            (status = 200, description = "成功获取货币型基金每日收益", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_money_fund_daily_em() -> Result<HttpResponse> {
    match fund::fund_money_fund_daily_em().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取东方财富场内交易型基金业绩排行
///
/// GET /api/v1/ak/fund/exchange_rank_em
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/exchange_rank_em",
        tag = "fund",
        responses(
            (status = 200, description = "成功获取场内基金排行", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_exchange_rank_em() -> Result<HttpResponse> {
    match fund::fund_exchange_rank_em().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取天天基金单只基金概况档案
///
/// GET /api/v1/ak/fund/overview_em
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/overview_em",
        tag = "fund",
        params(
            ("symbol" = Option<String>, Query, description = "基金代码，如 000001")
        ),
        responses(
            (status = 200, description = "成功获取基金概况档案", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_overview_em(
    query: web::Query<crate::models::ak::fortune::FortuneRankQuery>,
) -> Result<HttpResponse> {
    let symbol = query.symbol.clone().unwrap_or_else(|| "000001".to_string());
    match fund::fund_overview_em(&symbol).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取雪球(蛋卷)基金历史业绩表现数据
///
/// GET /api/v1/ak/fund/individual_achievement_xq
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/individual_achievement_xq",
        tag = "fund",
        params(
            ("symbol" = Option<String>, Query, description = "基金代码，如 000001")
        ),
        responses(
            (status = 200, description = "成功获取雪球基金业绩表现", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_individual_achievement_xq(
    query: web::Query<crate::models::ak::fortune::FortuneRankQuery>,
) -> Result<HttpResponse> {
    let symbol = query.symbol.clone().unwrap_or_else(|| "000001".to_string());
    match fund::fund_individual_achievement_xq(&symbol).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取同花顺单只基金基本信息明细
///
/// GET /api/v1/ak/fund/info_ths
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/info_ths",
        tag = "fund",
        params(
            ("symbol" = Option<String>, Query, description = "基金代码，如 000001")
        ),
        responses(
            (status = 200, description = "成功获取同花顺基金信息", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_info_ths(
    query: web::Query<crate::models::ak::fortune::FortuneRankQuery>,
) -> Result<HttpResponse> {
    let symbol = query.symbol.clone().unwrap_or_else(|| "000001".to_string());
    match fund::fund_info_ths(&symbol).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取雪球(蛋卷)基金持有不同天数的历史盈利概率预测
///
/// GET /api/v1/ak/fund/individual_profit_probability_xq
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/individual_profit_probability_xq",
        tag = "fund",
        params(
            ("symbol" = Option<String>, Query, description = "基金代码，如 000001")
        ),
        responses(
            (status = 200, description = "成功获取雪球盈利概率预测", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_individual_profit_probability_xq(
    query: web::Query<crate::models::ak::fortune::FortuneRankQuery>,
) -> Result<HttpResponse> {
    let symbol = query.symbol.clone().unwrap_or_else(|| "000001".to_string());
    match fund::fund_individual_profit_probability_xq(&symbol).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取雪球(蛋卷)基金资产配置详细占比分布
///
/// GET /api/v1/ak/fund/individual_detail_hold_xq
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/individual_detail_hold_xq",
        tag = "fund",
        params(
            ("symbol" = Option<String>, Query, description = "基金代码，如 000001")
        ),
        responses(
            (status = 200, description = "成功获取雪球持仓占比分布", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_individual_detail_hold_xq(
    query: web::Query<crate::models::ak::fortune::FortuneRankQuery>,
) -> Result<HttpResponse> {
    let symbol = query.symbol.clone().unwrap_or_else(|| "000001".to_string());
    match fund::fund_individual_detail_hold_xq(&symbol).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取天天基金货币型基金业绩排行
///
/// GET /api/v1/ak/fund/money_rank_em
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/money_rank_em",
        tag = "fund",
        responses(
            (status = 200, description = "成功获取货币型基金排行", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_money_rank_em() -> Result<HttpResponse> {
    match fund::fund_money_rank_em().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取天天基金理财型基金业绩排行
///
/// GET /api/v1/ak/fund/lcx_rank_em
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/lcx_rank_em",
        tag = "fund",
        responses(
            (status = 200, description = "成功获取理财型基金排行", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_lcx_rank_em() -> Result<HttpResponse> {
    match fund::fund_lcx_rank_em().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取天天基金香港基金业绩排行
///
/// GET /api/v1/ak/fund/hk_rank_em
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/hk_rank_em",
        tag = "fund",
        responses(
            (status = 200, description = "成功获取香港基金排行", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_hk_rank_em() -> Result<HttpResponse> {
    match fund::fund_hk_rank_em().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取证券投资基金业协会(AMAC)会员名录信息
///
/// GET /api/v1/ak/fund/amac_member_info
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/amac_member_info",
        tag = "fund",
        responses(
            (status = 200, description = "成功获取中基协会员名录", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_amac_member_info() -> Result<HttpResponse> {
    match fund::amac_member_info().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取证券投资基金业协会(AMAC)私募基金管理人登记公示信息
///
/// GET /api/v1/ak/fund/amac_manager_info
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/amac_manager_info",
        tag = "fund",
        responses(
            (status = 200, description = "成功获取中基协管理人信息", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_amac_manager_info() -> Result<HttpResponse> {
    match fund::amac_manager_info().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取天天基金分红公告
///
/// GET /api/v1/ak/fund/announcement_dividend_em
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/announcement_dividend_em",
        tag = "fund",
        responses(
            (status = 200, description = "成功获取基金分红公告", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_announcement_dividend_em() -> Result<HttpResponse> {
    match fund::fund_announcement_dividend_em().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取天天基金定期报告公告
///
/// GET /api/v1/ak/fund/announcement_report_em
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/announcement_report_em",
        tag = "fund",
        responses(
            (status = 200, description = "成功获取基金定期报告公告", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_announcement_report_em() -> Result<HttpResponse> {
    match fund::fund_announcement_report_em().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取天天基金人员变动公告数据
///
/// GET /api/v1/ak/fund/announcement_personnel_em
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/announcement_personnel_em",
        tag = "fund",
        responses(
            (status = 200, description = "成功获取基金人员变动公告", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_announcement_personnel_em() -> Result<HttpResponse> {
    match fund::fund_announcement_personnel_em().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取巨潮资讯基金定期报告行业配置明细
///
/// GET /api/v1/ak/fund/report_industry_allocation_cninfo
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/report_industry_allocation_cninfo",
        tag = "fund",
        responses(
            (status = 200, description = "成功获取巨潮基金行业配置", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_report_industry_allocation_cninfo() -> Result<HttpResponse> {
    match fund::fund_report_industry_allocation_cninfo().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取天天基金单只基金历年持仓重大变动明细
///
/// GET /api/v1/ak/fund/portfolio_change_em
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/portfolio_change_em",
        tag = "fund",
        params(
            ("symbol" = Option<String>, Query, description = "基金代码，如 000001"),
            ("year" = Option<String>, Query, description = "年份，如 2023")
        ),
        responses(
            (status = 200, description = "成功获取持仓重大变动数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_portfolio_change_em(
    query: web::Query<crate::models::ak::fortune::FortuneRankQuery>,
) -> Result<HttpResponse> {
    let symbol = query.symbol.clone().unwrap_or_else(|| "000001".to_string());
    let year = query.year.clone().unwrap_or_else(|| "2023".to_string());
    match fund::fund_portfolio_change_em(&symbol, &year).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取新浪财经 ETF 分类与实时行情
///
/// GET /api/v1/ak/fund/etf_category_sina
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/etf_category_sina",
        tag = "fund",
        params(
            ("symbol" = Option<String>, Query, description = "类型，如 股票型, QDII")
        ),
        responses(
            (status = 200, description = "成功获取新浪 ETF 分类行情", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_etf_category_sina(
    query: web::Query<crate::models::ak::fortune::FortuneRankQuery>,
) -> Result<HttpResponse> {
    let symbol = query.symbol.clone().unwrap_or_else(|| "股票型".to_string());
    match fund::fund_etf_category_sina(&symbol).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取上交所 ETF 规模分布
///
/// GET /api/v1/ak/fund/etf_scale_sse
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/etf_scale_sse",
        tag = "fund",
        responses(
            (status = 200, description = "成功获取上交所 ETF 规模", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_etf_scale_sse() -> Result<HttpResponse> {
    match fund::fund_etf_scale_sse().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取深交所 ETF 规模分布
///
/// GET /api/v1/ak/fund/etf_scale_szse
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/etf_scale_szse",
        tag = "fund",
        responses(
            (status = 200, description = "成功获取深交所 ETF 规模", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_etf_scale_szse() -> Result<HttpResponse> {
    match fund::fund_etf_scale_szse().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取新浪财经开放式基金规模
///
/// GET /api/v1/ak/fund/scale_open_sina
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/scale_open_sina",
        tag = "fund",
        responses(
            (status = 200, description = "成功获取新浪开放式基金规模", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_scale_open_sina() -> Result<HttpResponse> {
    match fund::fund_scale_open_sina().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取巨潮资讯基金定期报告资产配置明细
///
/// GET /api/v1/ak/fund/report_asset_allocation_cninfo
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/report_asset_allocation_cninfo",
        tag = "fund",
        responses(
            (status = 200, description = "成功获取巨潮基金资产配置", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_report_asset_allocation_cninfo() -> Result<HttpResponse> {
    match fund::fund_report_asset_allocation_cninfo().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取巨潮资讯基金定期报告股票持仓明细
///
/// GET /api/v1/ak/fund/report_stock_cninfo
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/report_stock_cninfo",
        tag = "fund",
        responses(
            (status = 200, description = "成功获取巨潮股票持仓明细", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_report_stock_cninfo() -> Result<HttpResponse> {
    match fund::fund_report_stock_cninfo().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取股票型基金仓位测算数据
///
/// GET /api/v1/ak/fund/stock_position_lg
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/stock_position_lg",
        tag = "fund",
        responses(
            (status = 200, description = "成功获取基金仓位测算数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_stock_position_lg() -> Result<HttpResponse> {
    match fund::fund_stock_position_lg().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取混合型基金仓位测算数据
///
/// GET /api/v1/ak/fund/balance_position_lg
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/balance_position_lg",
        tag = "fund",
        responses(
            (status = 200, description = "成功获取混合型基金仓位测算数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_balance_position_lg() -> Result<HttpResponse> {
    match fund::fund_balance_position_lg().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取灵活配置型基金仓位测算数据
///
/// GET /api/v1/ak/fund/linghuo_position_lg
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/linghuo_position_lg",
        tag = "fund",
        responses(
            (status = 200, description = "成功获取灵活配置型基金仓位测算数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_linghuo_position_lg() -> Result<HttpResponse> {
    match fund::fund_linghuo_position_lg().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取天天基金网上海证券基金评级数据
///
/// GET /api/v1/ak/fund/rating_sh
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/rating_sh",
        tag = "fund",
        responses(
            (status = 200, description = "成功获取上海证券基金评级", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_rating_sh() -> Result<HttpResponse> {
    match fund::fund_rating_sh().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取东方财富基金公司历年管理规模排行
///
/// GET /api/v1/ak/fund/aum_hist_em
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/aum_hist_em",
        tag = "fund",
        params(
            ("year" = Option<String>, Query, description = "年份，如 2023")
        ),
        responses(
            (status = 200, description = "成功获取历年管理规模排行", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_aum_hist_em(
    query: web::Query<crate::models::ak::fortune::FortuneRankQuery>,
) -> Result<HttpResponse> {
    let year = query.year.clone().unwrap_or_else(|| "2023".to_string());
    match fund::fund_aum_hist_em(&year).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取天天基金网招商证券基金评级数据
///
/// GET /api/v1/ak/fund/rating_zs
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/rating_zs",
        tag = "fund",
        responses(
            (status = 200, description = "成功获取招商证券基金评级", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_rating_zs() -> Result<HttpResponse> {
    match fund::fund_rating_zs().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取天天基金网济安金信基金评级数据
///
/// GET /api/v1/ak/fund/rating_ja
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/rating_ja",
        tag = "fund",
        responses(
            (status = 200, description = "成功获取济安金信基金评级", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_rating_ja() -> Result<HttpResponse> {
    match fund::fund_rating_ja().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取同花顺新发基金列表
///
/// GET /api/v1/ak/fund/new_found_ths
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/new_found_ths",
        tag = "fund",
        responses(
            (status = 200, description = "成功获取同花顺新发基金列表", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_new_found_ths() -> Result<HttpResponse> {
    match fund::fund_new_found_ths().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取雪球(蛋卷)基金个人分析与诊断报告
///
/// GET /api/v1/ak/fund/individual_analysis_xq
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/individual_analysis_xq",
        tag = "fund",
        params(
            ("symbol" = Option<String>, Query, description = "基金代码，如 000001")
        ),
        responses(
            (status = 200, description = "成功获取雪球基金分析报告", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_individual_analysis_xq(
    query: web::Query<crate::models::ak::fortune::FortuneRankQuery>,
) -> Result<HttpResponse> {
    let symbol = query.symbol.clone().unwrap_or_else(|| "000001".to_string());
    match fund::fund_individual_analysis_xq(&symbol).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取同花顺理财各分类(ETF/LOF/QDII等)基金每日净值与实时行情
///
/// GET /api/v1/ak/fund/etf_category_ths
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/etf_category_ths",
        tag = "fund",
        params(
            ("symbol" = Option<String>, Query, description = "基金类型，如 ETF, LOF, QDII, 股票型, 债券型")
        ),
        responses(
            (status = 200, description = "成功获取同花顺分类基金行情", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_etf_category_ths(
    query: web::Query<crate::models::ak::fortune::FortuneRankQuery>,
) -> Result<HttpResponse> {
    let symbol = query.symbol.clone().unwrap_or_else(|| "ETF".to_string());
    match fund::fund_etf_category_ths(&symbol).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取天天基金单只基金持仓债券明细
///
/// GET /api/v1/ak/fund/portfolio_bond_hold_em
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/portfolio_bond_hold_em",
        tag = "fund",
        params(
            ("symbol" = Option<String>, Query, description = "基金代码，如 000001"),
            ("year" = Option<String>, Query, description = "年份，如 2023")
        ),
        responses(
            (status = 200, description = "成功获取持仓债券明细", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_portfolio_bond_hold_em(
    query: web::Query<crate::models::ak::fortune::FortuneRankQuery>,
) -> Result<HttpResponse> {
    let symbol = query.symbol.clone().unwrap_or_else(|| "000001".to_string());
    let year = query.year.clone().unwrap_or_else(|| "2023".to_string());
    match fund::fund_portfolio_bond_hold_em(&symbol, &year).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取天天基金单只基金行业配置明细
///
/// GET /api/v1/ak/fund/portfolio_industry_allocation_em
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/portfolio_industry_allocation_em",
        tag = "fund",
        params(
            ("symbol" = Option<String>, Query, description = "基金代码，如 000001")
        ),
        responses(
            (status = 200, description = "成功获取行业配置明细", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_portfolio_industry_allocation_em(
    query: web::Query<crate::models::ak::fortune::FortuneRankQuery>,
) -> Result<HttpResponse> {
    let symbol = query.symbol.clone().unwrap_or_else(|| "000001".to_string());
    match fund::fund_portfolio_industry_allocation_em(&symbol).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取雪球蛋卷基金单只基金个人基本详细数据
///
/// GET /api/v1/ak/fund/individual_basic_info_xq
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/individual_basic_info_xq",
        tag = "fund",
        params(
            ("symbol" = Option<String>, Query, description = "基金代码，如 000001")
        ),
        responses(
            (status = 200, description = "成功获取雪球基金基本信息", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_individual_basic_info_xq(
    query: web::Query<crate::models::ak::fortune::FortuneRankQuery>,
) -> Result<HttpResponse> {
    let symbol = query.symbol.clone().unwrap_or_else(|| "000001".to_string());
    match fund::fund_individual_basic_info_xq(&symbol).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取天天基金指数型基金排行及基本数据
///
/// GET /api/v1/ak/fund/info_index_em
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fund/info_index_em",
        tag = "fund",
        params(
            ("symbol" = Option<String>, Query, description = "指数类型，如 沪深指数, 行业主题, 大盘指数"),
            ("indicator" = Option<String>, Query, description = "指数风格，如 被动指数型, 增强指数型")
        ),
        responses(
            (status = 200, description = "成功获取指数型基金列表", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fund_info_index_em(
    query: web::Query<crate::models::ak::fortune::FortuneRankQuery>,
) -> Result<HttpResponse> {
    let symbol = query
        .symbol
        .clone()
        .unwrap_or_else(|| "沪深指数".to_string());
    let indicator = query
        .indicator
        .clone()
        .unwrap_or_else(|| "被动指数型".to_string());
    match fund::fund_info_index_em(&symbol, &indicator).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/fund")
            .route("/name_em", web::get().to(get_fund_name_em))
            .route("/purchase_em", web::get().to(get_fund_purchase_em))
            .route("/etf_spot_em", web::get().to(get_fund_etf_spot_em))
            .route(
                "/open_fund_rank_em",
                web::get().to(get_fund_open_fund_rank_em),
            )
            .route("/manager_em", web::get().to(get_fund_manager_em))
            .route("/lof_spot_em", web::get().to(get_fund_lof_spot_em))
            .route(
                "/portfolio_hold_em",
                web::get().to(get_fund_portfolio_hold_em),
            )
            .route("/rating_all", web::get().to(get_fund_rating_all))
            .route("/fh_em", web::get().to(get_fund_fh_em))
            .route("/aum_em", web::get().to(get_fund_aum_em))
            .route("/aum_trend_em", web::get().to(get_fund_aum_trend_em))
            .route(
                "/value_estimation_em",
                web::get().to(get_fund_value_estimation_em),
            )
            .route("/new_found_em", web::get().to(get_fund_new_found_em))
            .route("/scale_change_em", web::get().to(get_fund_scale_change_em))
            .route(
                "/hold_structure_em",
                web::get().to(get_fund_hold_structure_em),
            )
            .route("/fee_em", web::get().to(get_fund_fee_em))
            .route("/etf_hist_em", web::get().to(get_fund_etf_hist_em))
            .route("/lof_hist_em", web::get().to(get_fund_lof_hist_em))
            .route(
                "/portfolio_bond_hold_em",
                web::get().to(get_fund_portfolio_bond_hold_em),
            )
            .route(
                "/portfolio_industry_allocation_em",
                web::get().to(get_fund_portfolio_industry_allocation_em),
            )
            .route(
                "/individual_basic_info_xq",
                web::get().to(get_fund_individual_basic_info_xq),
            )
            .route("/info_index_em", web::get().to(get_fund_info_index_em))
            .route(
                "/open_fund_daily_em",
                web::get().to(get_fund_open_fund_daily_em),
            )
            .route(
                "/money_fund_daily_em",
                web::get().to(get_fund_money_fund_daily_em),
            )
            .route(
                "/exchange_rank_em",
                web::get().to(get_fund_exchange_rank_em),
            )
            .route("/overview_em", web::get().to(get_fund_overview_em))
            .route(
                "/individual_achievement_xq",
                web::get().to(get_fund_individual_achievement_xq),
            )
            .route("/info_ths", web::get().to(get_fund_info_ths))
            .route(
                "/individual_profit_probability_xq",
                web::get().to(get_fund_individual_profit_probability_xq),
            )
            .route(
                "/individual_detail_hold_xq",
                web::get().to(get_fund_individual_detail_hold_xq),
            )
            .route("/money_rank_em", web::get().to(get_fund_money_rank_em))
            .route("/lcx_rank_em", web::get().to(get_fund_lcx_rank_em))
            .route("/hk_rank_em", web::get().to(get_fund_hk_rank_em))
            .route("/amac_member_info", web::get().to(get_amac_member_info))
            .route("/amac_manager_info", web::get().to(get_amac_manager_info))
            .route(
                "/announcement_dividend_em",
                web::get().to(get_fund_announcement_dividend_em),
            )
            .route(
                "/announcement_report_em",
                web::get().to(get_fund_announcement_report_em),
            )
            .route(
                "/announcement_personnel_em",
                web::get().to(get_fund_announcement_personnel_em),
            )
            .route(
                "/report_industry_allocation_cninfo",
                web::get().to(get_fund_report_industry_allocation_cninfo),
            )
            .route(
                "/portfolio_change_em",
                web::get().to(get_fund_portfolio_change_em),
            )
            .route(
                "/etf_category_sina",
                web::get().to(get_fund_etf_category_sina),
            )
            .route("/etf_scale_sse", web::get().to(get_fund_etf_scale_sse))
            .route("/etf_scale_szse", web::get().to(get_fund_etf_scale_szse))
            .route("/scale_open_sina", web::get().to(get_fund_scale_open_sina))
            .route(
                "/report_asset_allocation_cninfo",
                web::get().to(get_fund_report_asset_allocation_cninfo),
            )
            .route(
                "/report_stock_cninfo",
                web::get().to(get_fund_report_stock_cninfo),
            )
            .route(
                "/stock_position_lg",
                web::get().to(get_fund_stock_position_lg),
            )
            .route(
                "/balance_position_lg",
                web::get().to(get_fund_balance_position_lg),
            )
            .route(
                "/linghuo_position_lg",
                web::get().to(get_fund_linghuo_position_lg),
            )
            .route("/rating_sh", web::get().to(get_fund_rating_sh))
            .route("/rating_zs", web::get().to(get_fund_rating_zs))
            .route("/rating_ja", web::get().to(get_fund_rating_ja))
            .route("/aum_hist_em", web::get().to(get_fund_aum_hist_em))
            .route("/new_found_ths", web::get().to(get_fund_new_found_ths))
            .route(
                "/individual_analysis_xq",
                web::get().to(get_fund_individual_analysis_xq),
            )
            .route(
                "/etf_category_ths",
                web::get().to(get_fund_etf_category_ths),
            )
            .route("/cf_em", web::get().to(get_fund_cf_em))
            .route("/fh_rank_em", web::get().to(get_fund_fh_rank_em))
            .route(
                "/scale_close_sina",
                web::get().to(get_fund_scale_close_sina),
            ),
    );
}
