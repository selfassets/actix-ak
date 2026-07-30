//! 债券数据 HTTP 处理器
//!
//! 提供可转债、中国/美国国债收益率等数据端点

use crate::models::{ak::bond::BondQuery, ApiResponse};
use crate::services::ak::bond;
use actix_web::{web, HttpResponse, Result};

/// 获取沪深可转债实时行情
///
/// GET /api/v1/ak/bond/zh_cov_spot
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/bond/zh_cov_spot",
        tag = "bond",
        responses(
            (status = 200, description = "成功获取沪深可转债实时行情", body = ApiResponse<Vec<BondZhCovSpotItem>>)
        )
    )
)]
pub async fn get_bond_zh_cov_spot() -> Result<HttpResponse> {
    match bond::get_bond_zh_cov_spot().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取中国国债收益率历史数据（新浪源）
///
/// GET /api/v1/ak/bond/gb_zh_sina
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/bond/gb_zh_sina",
        tag = "bond",
        params(
            BondQuery
        ),
        responses(
            (status = 200, description = "成功获取中国国债收益率数据", body = ApiResponse<Vec<BondGbKlineItem>>)
        )
    )
)]
pub async fn get_bond_gb_zh_sina(query: web::Query<BondQuery>) -> Result<HttpResponse> {
    match bond::get_bond_gb_zh_sina(query.into_inner()).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取美国国债收益率历史数据（新浪源）
///
/// GET /api/v1/ak/bond/gb_us_sina
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/bond/gb_us_sina",
        tag = "bond",
        params(
            BondQuery
        ),
        responses(
            (status = 200, description = "成功获取美国国债收益率数据", body = ApiResponse<Vec<BondGbKlineItem>>)
        )
    )
)]
pub async fn get_bond_gb_us_sina(query: web::Query<BondQuery>) -> Result<HttpResponse> {
    match bond::get_bond_gb_us_sina(query.into_inner()).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取中美国债收益率对比数据（东方财富）
///
/// GET /api/v1/ak/bond/zh_us_rate
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/bond/zh_us_rate",
        tag = "bond",
        responses(
            (status = 200, description = "成功获取中美国债收益率数据", body = ApiResponse<Vec<BondZhUsRateItem>>)
        )
    )
)]
pub async fn get_bond_zh_us_rate() -> Result<HttpResponse> {
    match bond::get_bond_zh_us_rate().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取上证质押式国债逆回购行情（东方财富）
///
/// GET /api/v1/ak/bond/sh_buy_back
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/bond/sh_buy_back",
        tag = "bond",
        responses(
            (status = 200, description = "成功获取上证质押式国债逆回购行情", body = ApiResponse<Vec<BondBuyBackItem>>)
        )
    )
)]
pub async fn get_bond_sh_buy_back() -> Result<HttpResponse> {
    match bond::get_bond_sh_buy_back().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取深证质押式国债逆回购行情（东方财富）
///
/// GET /api/v1/ak/bond/sz_buy_back
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/bond/sz_buy_back",
        tag = "bond",
        responses(
            (status = 200, description = "成功获取深证质押式国债逆回购行情", body = ApiResponse<Vec<BondBuyBackItem>>)
        )
    )
)]
pub async fn get_bond_sz_buy_back() -> Result<HttpResponse> {
    match bond::get_bond_sz_buy_back().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取集思录可转债等权指数历史
///
/// GET /api/v1/ak/bond/cb_index_jsl
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/bond/cb_index_jsl",
        tag = "bond",
        responses(
            (status = 200, description = "成功获取集思录可转债等权指数", body = ApiResponse<Vec<BondJslItem>>)
        )
    )
)]
pub async fn get_bond_cb_index_jsl() -> Result<HttpResponse> {
    match bond::get_bond_cb_index_jsl().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取集思录可转债强赎信息列表
///
/// GET /api/v1/ak/bond/cb_redeem_jsl
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/bond/cb_redeem_jsl",
        tag = "bond",
        responses(
            (status = 200, description = "成功获取集思录可转债强赎列表", body = ApiResponse<Vec<BondJslItem>>)
        )
    )
)]
pub async fn get_bond_cb_redeem_jsl() -> Result<HttpResponse> {
    match bond::get_bond_cb_redeem_jsl().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取新浪财经可转债详情资料
///
/// GET /api/v1/ak/bond/cb_profile_sina
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/bond/cb_profile_sina",
        tag = "bond",
        params(
            BondQuery
        ),
        responses(
            (status = 200, description = "成功获取新浪可转债详情资料", body = ApiResponse<Vec<BondCbProfileItem>>)
        )
    )
)]
pub async fn get_bond_cb_profile_sina(query: web::Query<BondQuery>) -> Result<HttpResponse> {
    match bond::get_bond_cb_profile_sina(query.into_inner()).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取东方财富网可转债比价表数据
///
/// GET /api/v1/ak/bond/cov_comparison
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/bond/cov_comparison",
        tag = "bond",
        responses(
            (status = 200, description = "成功获取可转债比价表", body = ApiResponse<Vec<BondCovComparisonItem>>)
        )
    )
)]
pub async fn get_bond_cov_comparison() -> Result<HttpResponse> {
    match bond::get_bond_cov_comparison().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 配置债券路由
///
/// 挂载路径：/bond
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/bond")
            .route("/zh_cov_spot", web::get().to(get_bond_zh_cov_spot))
            .route("/gb_zh_sina", web::get().to(get_bond_gb_zh_sina))
            .route("/gb_us_sina", web::get().to(get_bond_gb_us_sina))
            .route("/zh_us_rate", web::get().to(get_bond_zh_us_rate))
            .route("/sh_buy_back", web::get().to(get_bond_sh_buy_back))
            .route("/sz_buy_back", web::get().to(get_bond_sz_buy_back))
            .route("/cb_index_jsl", web::get().to(get_bond_cb_index_jsl))
            .route("/cb_redeem_jsl", web::get().to(get_bond_cb_redeem_jsl))
            .route("/cb_profile_sina", web::get().to(get_bond_cb_profile_sina))
            .route("/cov_comparison", web::get().to(get_bond_cov_comparison)),
    );
}
