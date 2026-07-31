//! 百度地图慧眼-百度迁徙 HTTP 处理器

use actix_web::{web, HttpResponse, Result};
use crate::models::ApiResponse;
use crate::services::ak::migration;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct BaiduMigrationQuery {
    pub area: Option<String>,
    pub indicator: Option<String>,
    pub date: Option<String>,
}

/// 获取百度迁徙-XXX迁入地/迁出地 Top100 详情
///
/// GET /api/v1/ak/event/migration_area_baidu
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/event/migration_area_baidu",
        tag = "event",
        params(BaiduMigrationQuery),
        responses(
            (status = 200, description = "成功获取百度迁徙地区Top100详情", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_migration_area_baidu(query: web::Query<BaiduMigrationQuery>) -> Result<HttpResponse> {
    let q = query.into_inner();
    let area = q.area.unwrap_or_else(|| "重庆市".to_string());
    let indicator = q.indicator.unwrap_or_else(|| "move_in".to_string());
    let date = q.date.unwrap_or_else(|| "20230922".to_string());
    
    match migration::get_migration_area_baidu(&area, &indicator, &date).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取百度迁徙-迁徙规模历史曲线指数
///
/// GET /api/v1/ak/event/migration_scale_baidu
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/event/migration_scale_baidu",
        tag = "event",
        params(BaiduMigrationQuery),
        responses(
            (status = 200, description = "成功获取百度迁徙规模曲线指数", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_migration_scale_baidu(query: web::Query<BaiduMigrationQuery>) -> Result<HttpResponse> {
    let q = query.into_inner();
    let area = q.area.unwrap_or_else(|| "广州市".to_string());
    let indicator = q.indicator.unwrap_or_else(|| "move_in".to_string());
    
    match migration::get_migration_scale_baidu(&area, &indicator).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/event")
            .route("/migration_area_baidu", web::get().to(get_migration_area_baidu))
            .route("/migration_scale_baidu", web::get().to(get_migration_scale_baidu))
    );
}
