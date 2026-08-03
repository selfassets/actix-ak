//! 富豪排行榜 (Fortune) 相关 HTTP 处理器

use crate::models::ak::fortune::FortuneRankQuery;
use crate::models::ApiResponse;
use crate::services::ak::fortune;
use actix_web::{web, HttpResponse, Result};

/// 获取 500 强历年排行
///
/// GET /api/v1/ak/fortune/rank
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fortune/rank",
        tag = "fortune",
        params(FortuneRankQuery),
        responses(
            (status = 200, description = "成功", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_fortune_rank(query: web::Query<FortuneRankQuery>) -> Result<HttpResponse> {
    let year = query.year.clone().unwrap_or_else(|| "2023".to_string());
    match fortune::fortune_rank(&year).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取彭博亿万富豪指数
///
/// GET /api/v1/ak/fortune/bloomberg_billionaires
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fortune/bloomberg_billionaires",
        tag = "fortune",
        responses(
            (status = 200, description = "成功", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_index_bloomberg_billionaires() -> Result<HttpResponse> {
    match fortune::index_bloomberg_billionaires().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取历史彭博亿万富豪指数
///
/// GET /api/v1/ak/fortune/bloomberg_billionaires_hist
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fortune/bloomberg_billionaires_hist",
        tag = "fortune",
        params(FortuneRankQuery),
        responses(
            (status = 200, description = "成功", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_index_bloomberg_billionaires_hist(
    query: web::Query<FortuneRankQuery>,
) -> Result<HttpResponse> {
    let year = query.year.clone().unwrap_or_else(|| "2021".to_string());
    match fortune::index_bloomberg_billionaires_hist(&year).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取福布斯中国富豪榜单
///
/// GET /api/v1/ak/fortune/forbes_rank
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fortune/forbes_rank",
        tag = "fortune",
        params(FortuneRankQuery),
        responses(
            (status = 200, description = "成功", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_forbes_rank(query: web::Query<FortuneRankQuery>) -> Result<HttpResponse> {
    let symbol = query
        .symbol
        .clone()
        .unwrap_or_else(|| "2021福布斯中国创投人100".to_string());
    match fortune::forbes_rank(&symbol).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取胡润各类富豪排行榜
///
/// GET /api/v1/ak/fortune/hurun_rank
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fortune/hurun_rank",
        tag = "fortune",
        params(FortuneRankQuery),
        responses(
            (status = 200, description = "成功", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_hurun_rank(query: web::Query<FortuneRankQuery>) -> Result<HttpResponse> {
    let indicator = query
        .indicator
        .clone()
        .unwrap_or_else(|| "胡润百富榜".to_string());
    let year = query.year.clone().unwrap_or_else(|| "2023".to_string());
    match fortune::hurun_rank(&indicator, &year).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取新财富 500 富人榜
///
/// GET /api/v1/ak/fortune/xincaifu_rank
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fortune/xincaifu_rank",
        tag = "fortune",
        params(FortuneRankQuery),
        responses(
            (status = 200, description = "成功", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_xincaifu_rank(query: web::Query<FortuneRankQuery>) -> Result<HttpResponse> {
    let year = query.year.clone().unwrap_or_else(|| "2022".to_string());
    match fortune::xincaifu_rank(&year).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/fortune")
            .route("/rank", web::get().to(get_fortune_rank))
            .route(
                "/bloomberg_billionaires",
                web::get().to(get_index_bloomberg_billionaires),
            )
            .route(
                "/bloomberg_billionaires_hist",
                web::get().to(get_index_bloomberg_billionaires_hist),
            )
            .route("/forbes_rank", web::get().to(get_forbes_rank))
            .route("/hurun_rank", web::get().to(get_hurun_rank))
            .route("/xincaifu_rank", web::get().to(get_xincaifu_rank)),
    );
}
