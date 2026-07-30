//! 拆借利率与利率数据 HTTP 处理器

use crate::models::{ak::interest_rate::InterbankRateQuery, ApiResponse};
use crate::services::ak::interest_rate;
use actix_web::{web, HttpResponse, Result};

/// 获取银行间同业拆借利率历史数据
///
/// GET /api/v1/ak/interest_rate/rate_interbank
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/interest_rate/rate_interbank",
        tag = "interest_rate",
        params(
            InterbankRateQuery
        ),
        responses(
            (status = 200, description = "成功获取银行间拆借利率数据", body = ApiResponse<Vec<InterbankRateItem>>)
        )
    )
)]
pub async fn get_rate_interbank(query: web::Query<InterbankRateQuery>) -> Result<HttpResponse> {
    match interest_rate::get_rate_interbank(query.into_inner()).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 配置拆借利率与利率路由
///
/// 挂载路径：/interest_rate
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/interest_rate").route("/rate_interbank", web::get().to(get_rate_interbank)),
    );
}
