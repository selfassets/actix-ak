//! 银行与金融监管数据 HTTP 处理器
//!
//! 提供行政处罚公开表等数据端点

use crate::models::{ak::bank::BankFjcfQuery, ApiResponse};
use crate::services::ak::bank;
use actix_web::{web, HttpResponse, Result};

/// 获取行政处罚数据总条数
///
/// GET /api/v1/ak/bank/fjcf_total_num
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/bank/fjcf_total_num",
        tag = "bank",
        params(
            BankFjcfQuery
        ),
        responses(
            (status = 200, description = "成功获取行政处罚总条数", body = ApiResponse<i64>)
        )
    )
)]
pub async fn get_fjcf_total_num(query: web::Query<BankFjcfQuery>) -> Result<HttpResponse> {
    match bank::get_bank_fjcf_total_num(query.item.clone()).await {
        Ok(num) => Ok(HttpResponse::Ok().json(ApiResponse::success(num))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取行政处罚数据总页数
///
/// GET /api/v1/ak/bank/fjcf_total_page
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/bank/fjcf_total_page",
        tag = "bank",
        params(
            BankFjcfQuery
        ),
        responses(
            (status = 200, description = "成功获取行政处罚总页数", body = ApiResponse<i64>)
        )
    )
)]
pub async fn get_fjcf_total_page(query: web::Query<BankFjcfQuery>) -> Result<HttpResponse> {
    match bank::get_bank_fjcf_total_page(query.item.clone(), query.begin).await {
        Ok(page_count) => Ok(HttpResponse::Ok().json(ApiResponse::success(page_count))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取行政处罚列表概要
///
/// GET /api/v1/ak/bank/fjcf_list
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/bank/fjcf_list",
        tag = "bank",
        params(
            BankFjcfQuery
        ),
        responses(
            (status = 200, description = "成功获取行政处罚列表", body = ApiResponse<Vec<BankFjcfListItem>>)
        )
    )
)]
pub async fn get_fjcf_list(query: web::Query<BankFjcfQuery>) -> Result<HttpResponse> {
    match bank::get_bank_fjcf_list(query.page, query.item.clone(), query.begin).await {
        Ok(list) => Ok(HttpResponse::Ok().json(ApiResponse::success(list))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取行政处罚信息公开表详情
///
/// GET /api/v1/ak/bank/fjcf_detail
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/bank/fjcf_detail",
        tag = "bank",
        params(
            BankFjcfQuery
        ),
        responses(
            (status = 200, description = "成功获取行政处罚公开表详情", body = ApiResponse<Vec<BankFjcfDetailItem>>)
        )
    )
)]
pub async fn get_fjcf_detail(query: web::Query<BankFjcfQuery>) -> Result<HttpResponse> {
    match bank::get_bank_fjcf_detail(query.page, query.item.clone(), query.begin).await {
        Ok(details) => Ok(HttpResponse::Ok().json(ApiResponse::success(details))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 配置银行与金融监管路由
///
/// 挂载路径：/bank
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/bank")
            .route("/fjcf_total_num", web::get().to(get_fjcf_total_num))
            .route("/fjcf_total_page", web::get().to(get_fjcf_total_page))
            .route("/fjcf_list", web::get().to(get_fjcf_list))
            .route("/fjcf_detail", web::get().to(get_fjcf_detail)),
    );
}
