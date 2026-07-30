//! 计算与波动率估算 HTTP 处理器

use crate::models::{ak::cal::OhlcItem, ApiResponse};
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

/// 配置计算工具路由
///
/// 挂载路径：/cal
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/cal")
            .route("/volatility_yz", web::post().to(calculate_volatility_yz)),
    );
}
