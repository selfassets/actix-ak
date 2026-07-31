//! 国家统计局与国家杠杆率、外汇情绪相关 HTTP 处理器

use crate::models::ApiResponse;
use crate::services::ak::macro_cnbs;
use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct FxSentimentQuery {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NbsNationQuery {
    pub kind: Option<String>,
    pub path: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NbsRegionQuery {
    pub kind: Option<String>,
    pub path: Option<String>,
    pub region: Option<String>,
}

/// 获取中国宏观杠杆率数据
///
/// GET /api/v1/ak/macro/cnbs
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/cnbs",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取杠杆率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_cnbs() -> Result<HttpResponse> {
    match macro_cnbs::get_macro_cnbs().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取外汇多空情绪数据
///
/// GET /api/v1/ak/macro/fx_sentiment
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/fx_sentiment",
        tag = "macro",
        params(FxSentimentQuery),
        responses(
            (status = 200, description = "成功获取外汇情绪数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_fx_sentiment(query: web::Query<FxSentimentQuery>) -> Result<HttpResponse> {
    let q = query.into_inner();
    let start_date = q.start_date.unwrap_or_else(|| "20231011".to_string());
    let end_date = q.end_date.unwrap_or_else(|| "20231017".to_string());
    match macro_cnbs::get_macro_fx_sentiment(&start_date, &end_date).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取国家统计局全国多维时间序列大表数据
///
/// GET /api/v1/ak/macro/china_nbs_nation
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/china_nbs_nation",
        tag = "macro",
        params(NbsNationQuery),
        responses(
            (status = 200, description = "成功获取国家统计局多维数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_china_nbs_nation(query: web::Query<NbsNationQuery>) -> Result<HttpResponse> {
    let q = query.into_inner();
    let kind = q.kind.unwrap_or_else(|| "月度数据".to_string());
    let path = q.path.unwrap_or_else(|| "A0101".to_string());
    match macro_cnbs::get_macro_china_nbs_nation(&kind, &path).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取国家统计局各省份省级多维地区及城市房价大表数据
///
/// GET /api/v1/ak/macro/china_nbs_region
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/china_nbs_region",
        tag = "macro",
        params(NbsRegionQuery),
        responses(
            (status = 200, description = "成功获取国家统计局省级及城市价格多维数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_china_nbs_region(query: web::Query<NbsRegionQuery>) -> Result<HttpResponse> {
    let q = query.into_inner();
    let kind = q.kind.unwrap_or_else(|| "分省月度数据".to_string());
    let path = q.path.unwrap_or_else(|| "A0101".to_string());
    let region = q.region.clone();
    match macro_cnbs::get_macro_china_nbs_region(&kind, &path, region).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/macro")
            .route("/cnbs", web::get().to(get_macro_cnbs))
            .route("/fx_sentiment", web::get().to(get_macro_fx_sentiment))
            .route(
                "/china_nbs_nation",
                web::get().to(get_macro_china_nbs_nation),
            )
            .route(
                "/china_nbs_region",
                web::get().to(get_macro_china_nbs_region),
            ),
    );
}
