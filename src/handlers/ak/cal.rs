//! 计算与波动率估算 HTTP 处理器

use crate::models::{ak::cal::OhlcItem, ak::cal::RvMinuteQuery, ApiResponse};
use crate::services::ak::cal;
use actix_web::{web, HttpResponse, Result};

/// 计算 Yang-Zhang 已实现波动率
///
/// POST /api/v1/ak/cal/volatility_yz
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        post,
        path = "/ak/cal/volatility_yz",
        tag = "cal",
        request_body = Vec<OhlcItem>,
        responses(
            (status = 200, description = "成功计算 Yang-Zhang 已实现波动率", body = ApiResponse<YangZhangVolatilityResult>)
        )
    )
)]
pub async fn calculate_volatility_yz(body: web::Json<Vec<OhlcItem>>) -> Result<HttpResponse> {
    match cal::calculate_yang_zhang_volatility(&body) {
        Ok(result) => Ok(HttpResponse::Ok().json(ApiResponse::success(result))),
        Err(err) => Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取股票分钟级行情并清洗为 YZ 波动率输入格式
///
/// GET /api/v1/ak/cal/rv_stock_em
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/cal/rv_stock_em",
        tag = "cal",
        params(
            RvMinuteQuery
        ),
        responses(
            (status = 200, description = "成功获取并清洗股票分钟行情", body = ApiResponse<Vec<OhlcItem>>)
        )
    )
)]
pub async fn get_rv_stock_em(query: web::Query<RvMinuteQuery>) -> Result<HttpResponse> {
    match cal::rv_from_stock_zh_a_hist_min_em(query.into_inner()).await {
        Ok(result) => Ok(HttpResponse::Ok().json(ApiResponse::success(result))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取期货分钟级行情并清洗为 YZ 波动率输入格式
///
/// GET /api/v1/ak/cal/rv_futures_sina
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/cal/rv_futures_sina",
        tag = "cal",
        params(
            RvMinuteQuery
        ),
        responses(
            (status = 200, description = "成功获取并清洗期货分钟行情", body = ApiResponse<Vec<OhlcItem>>)
        )
    )
)]
pub async fn get_rv_futures_sina(query: web::Query<RvMinuteQuery>) -> Result<HttpResponse> {
    match cal::rv_from_futures_zh_minute_sina(query.into_inner()).await {
        Ok(result) => Ok(HttpResponse::Ok().json(ApiResponse::success(result))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 配置计算工具路由
///
/// 挂载路径：/cal
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/cal")
            .route("/volatility_yz", web::post().to(calculate_volatility_yz))
            .route("/rv_stock_em", web::get().to(get_rv_stock_em))
            .route("/rv_futures_sina", web::get().to(get_rv_futures_sina)),
    );
}
