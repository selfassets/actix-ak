//! AkShare (AK) 请求处理器
//!
//! 提供 AK 模块相关的 HTTP 端点

use crate::models::{ak::EpuIndexQuery, ApiResponse};
use crate::services::ak;
use actix_web::{web, HttpResponse, Result};

/// 获取 AK 模块元数据与信息
///
/// GET /api/v1/ak/info
/// 返回 AK 接口服务的服务信息与分类
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/info",
        tag = "ak",
        responses(
            (status = 200, description = "成功获取 AK 信息", body = ApiResponse<AkInfo>)
        )
    )
)]
pub async fn get_info() -> Result<HttpResponse> {
    match ak::get_ak_info().await {
        Ok(info) => Ok(HttpResponse::Ok().json(ApiResponse::success(info))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取经济政策不确定性指数（EPU Index）
///
/// GET /api/v1/ak/article_epu_index
/// 可传入 symbol 参数（例如 China, USA, Hong Kong 等，默认为 China）
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/article_epu_index",
        tag = "ak",
        params(
            EpuIndexQuery
        ),
        responses(
            (status = 200, description = "成功获取经济政策不确定性指数数据", body = ApiResponse<Vec<EpuIndexItem>>)
        )
    )
)]
pub async fn get_article_epu_index(query: web::Query<EpuIndexQuery>) -> Result<HttpResponse> {
    match ak::get_article_epu_index(query.symbol.clone()).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 配置 AK 模块路由
///
/// 挂载路径：/ak
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/ak")
            .route("/info", web::get().to(get_info))
            .route("/article_epu_index", web::get().to(get_article_epu_index)),
    );
}
