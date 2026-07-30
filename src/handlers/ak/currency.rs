//! 外汇与货币数据 HTTP 处理器

use crate::models::{ak::currency::CurrencyBocQuery, ApiResponse};
use crate::services::ak::currency;
use actix_web::{web, HttpResponse, Result};

/// 获取新浪财经-中国银行人民币牌价历史数据
///
/// GET /api/v1/ak/currency/boc_sina
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/currency/boc_sina",
        tag = "currency",
        params(
            CurrencyBocQuery
        ),
        responses(
            (status = 200, description = "成功获取中国银行人民币牌价历史数据", body = ApiResponse<Vec<CurrencyBocItem>>)
        )
    )
)]
pub async fn get_currency_boc_sina(query: web::Query<CurrencyBocQuery>) -> Result<HttpResponse> {
    match currency::get_currency_boc_sina(query.into_inner()).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取国家外汇管理局 (SAFE) 人民币汇率中间价数据
///
/// GET /api/v1/ak/currency/boc_safe
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/currency/boc_safe",
        tag = "currency",
        params(
            CurrencyBocQuery
        ),
        responses(
            (status = 200, description = "成功获取国家外汇管理局人民币汇率中间价", body = ApiResponse<Vec<CurrencySafeItem>>)
        )
    )
)]
pub async fn get_currency_boc_safe(query: web::Query<CurrencyBocQuery>) -> Result<HttpResponse> {
    match currency::get_currency_boc_safe(query.into_inner()).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 配置外汇与货币路由
///
/// 挂载路径：/currency
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/currency")
            .route("/boc_sina", web::get().to(get_currency_boc_sina))
            .route("/boc_safe", web::get().to(get_currency_boc_safe)),
    );
}
