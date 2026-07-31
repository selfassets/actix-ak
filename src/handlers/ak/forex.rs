//! 东方财富网外汇行情接口处理器

use crate::models::ak::forex::ForexQuery;
use crate::models::ApiResponse;
use crate::services::ak::forex;
use actix_web::{web, HttpResponse, Result};

/// 获取东方财富外汇实时各指标数据
///
/// GET /api/v1/ak/forex_spot_em
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/forex_spot_em",
        tag = "forex",
        responses(
            (status = 200, description = "成功", body = ApiResponse<Vec<Value>>)
        )
    )
)]
pub async fn get_forex_spot_em() -> Result<HttpResponse> {
    match forex::forex_spot_em().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取东方财富外汇高频历史日K数据
///
/// GET /api/v1/ak/forex_hist_em
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/forex_hist_em",
        tag = "forex",
        params(ForexQuery),
        responses(
            (status = 200, description = "成功", body = ApiResponse<Vec<Value>>)
        )
    )
)]
pub async fn get_forex_hist_em(query: web::Query<ForexQuery>) -> Result<HttpResponse> {
    let symbol = query.symbol.clone().unwrap_or_else(|| "USDCNH".to_string());
    match forex::forex_hist_em(&symbol).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/forex")
            .route("/spot_em", web::get().to(get_forex_spot_em))
            .route("/hist_em", web::get().to(get_forex_hist_em)),
    );
}
