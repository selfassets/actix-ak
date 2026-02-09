//! 健康检查接口
//!
//! 用于监控服务运行状态

use crate::models::ApiResponse;
use actix_web::{web, HttpResponse, Result};

/// 健康检查处理函数
///
/// GET /api/v1/health
/// 返回服务运行状态
#[cfg_attr(feature = "swagger", utoipa::path(
    get,
    path = "/health",
    tag = "health",
    responses(
        (status = 200, description = "服务运行正常", body = ApiResponse<String>)
    )
))]
pub async fn health_check() -> Result<HttpResponse> {
    let response = ApiResponse::success("服务运行正常");
    Ok(HttpResponse::Ok().json(response))
}

/// 配置健康检查路由
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check));
}
