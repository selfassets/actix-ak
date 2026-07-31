//! 能源(Energy)相关 HTTP 处理器

use crate::models::{ak::energy::EnergyOilQuery, ApiResponse};
use crate::services::ak::energy;
use actix_web::{web, HttpResponse, Result};

/// 获取汽柴油历史调价信息
///
/// GET /api/v1/ak/energy_oil_hist
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/energy_oil_hist",
        tag = "energy",
        responses(
            (status = 200, description = "成功", body = ApiResponse<Vec<Value>>)
        )
    )
)]
pub async fn get_energy_oil_hist() -> Result<HttpResponse> {
    match energy::energy_oil_hist().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取汽柴油调价地区明细
///
/// GET /api/v1/ak/energy_oil_detail
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/energy_oil_detail",
        tag = "energy",
        params(EnergyOilQuery),
        responses(
            (status = 200, description = "成功", body = ApiResponse<Vec<Value>>)
        )
    )
)]
pub async fn get_energy_oil_detail(query: web::Query<EnergyOilQuery>) -> Result<HttpResponse> {
    let q = query.into_inner();
    let date = q.date.unwrap_or_else(|| "2024-01-18".to_string());
    match energy::energy_oil_detail(&date).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取国内碳排放交易行情
///
/// GET /api/v1/ak/energy_carbon_domestic
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/energy_carbon_domestic",
        tag = "energy",
        params(
            ("symbol" = Option<String>, Query, description = "地点，如 湖北、北京、上海 等")
        ),
        responses(
            (status = 200, description = "成功", body = ApiResponse<Vec<Value>>)
        )
    )
)]
pub async fn get_energy_carbon_domestic(query: web::Query<EnergyOilQuery>) -> Result<HttpResponse> {
    let symbol = query.symbol.clone().unwrap_or_else(|| "湖北".to_string());
    match energy::energy_carbon_domestic(&symbol).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取北京碳交易所公开交易数据
///
/// GET /api/v1/ak/energy_carbon_bj
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/energy_carbon_bj",
        tag = "energy",
        responses(
            (status = 200, description = "成功", body = ApiResponse<Vec<Value>>)
        )
    )
)]
pub async fn get_energy_carbon_bj() -> Result<HttpResponse> {
    match energy::energy_carbon_bj().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取深圳地区国内碳交易所交易数据
///
/// GET /api/v1/ak/energy_carbon_sz
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/energy_carbon_sz",
        tag = "energy",
        responses(
            (status = 200, description = "成功", body = ApiResponse<Vec<Value>>)
        )
    )
)]
pub async fn get_energy_carbon_sz() -> Result<HttpResponse> {
    match energy::energy_carbon_sz().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取欧盟国际碳情交易数据
///
/// GET /api/v1/ak/energy_carbon_eu
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/energy_carbon_eu",
        tag = "energy",
        responses(
            (status = 200, description = "成功", body = ApiResponse<Vec<Value>>)
        )
    )
)]
pub async fn get_energy_carbon_eu() -> Result<HttpResponse> {
    match energy::energy_carbon_eu().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/energy")
            .route("/oil_hist", web::get().to(get_energy_oil_hist))
            .route("/oil_detail", web::get().to(get_energy_oil_detail))
            .route(
                "/carbon_domestic",
                web::get().to(get_energy_carbon_domestic),
            )
            .route("/carbon_bj", web::get().to(get_energy_carbon_bj))
            .route("/carbon_sz", web::get().to(get_energy_carbon_sz))
            .route("/carbon_eu", web::get().to(get_energy_carbon_eu)),
    );
}
