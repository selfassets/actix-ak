//! AkShare (AK) 请求处理器
//!
//! 提供 AK 模块相关的 HTTP 端点

pub mod bank;
pub mod bond;
pub mod cal;
pub mod crypto;
pub mod currency;
pub mod energy;
pub mod forex;
pub mod fortune;
pub mod fund;
pub mod futures_settle;
pub mod interest_rate;
pub mod macro_cnbs;
pub mod macro_data;
pub mod migration;

use crate::models::{
    ak::EpuIndexQuery, ak::FredQuery, ak::OmanRvQuery, ak::OmanRvShortQuery, ak::RlabRvQuery,
    ApiResponse,
};
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

/// 获取美联储 FRED-MD (月度) 宏观经济数据
///
/// GET /api/v1/ak/fred_md
/// 可传入 date 参数（例如 "2020-01", "2023-03"，默认为 "2020-01"）
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fred_md",
        tag = "ak",
        params(
            FredQuery
        ),
        responses(
            (status = 200, description = "成功获取 FRED-MD 月度数据", body = ApiResponse<Vec<FredItem>>)
        )
    )
)]
pub async fn get_fred_md(query: web::Query<FredQuery>) -> Result<HttpResponse> {
    match ak::fred_md(query.date.clone()).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取美联储 FRED-QD (季度) 宏观经济数据
///
/// GET /api/v1/ak/fred_qd
/// 可传入 date 参数（例如 "2020-01", "2023-03"，默认为 "2020-01"）
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/fred_qd",
        tag = "ak",
        params(
            FredQuery
        ),
        responses(
            (status = 200, description = "成功获取 FRED-QD 季度数据", body = ApiResponse<Vec<FredItem>>)
        )
    )
)]
pub async fn get_fred_qd(query: web::Query<FredQuery>) -> Result<HttpResponse> {
    match ak::fred_qd(query.date.clone()).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取 Oxford-Man 研究所 Realized Volatility 数据
///
/// GET /api/v1/ak/article_oman_rv
/// 可传入 symbol, index 参数
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/article_oman_rv",
        tag = "ak",
        params(
            OmanRvQuery
        ),
        responses(
            (status = 200, description = "成功获取 Oxford-Man 实际波动率数据", body = ApiResponse<Vec<VolatilityItem>>)
        )
    )
)]
pub async fn get_article_oman_rv(query: web::Query<OmanRvQuery>) -> Result<HttpResponse> {
    match ak::article_oman_rv(query.symbol.clone(), query.index.clone()).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取 Oxford-Man 研究所 Realized Volatility 简易数据
///
/// GET /api/v1/ak/article_oman_rv_short
/// 可传入 symbol 参数
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/article_oman_rv_short",
        tag = "ak",
        params(
            OmanRvShortQuery
        ),
        responses(
            (status = 200, description = "成功获取 Oxford-Man 简易实际波动率数据", body = ApiResponse<Vec<VolatilityItem>>)
        )
    )
)]
pub async fn get_article_oman_rv_short(
    query: web::Query<OmanRvShortQuery>,
) -> Result<HttpResponse> {
    match ak::article_oman_rv_short(query.symbol.clone()).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取修大成主页 Risk Lab - Realized Volatility 数据
///
/// GET /api/v1/ak/article_rlab_rv
/// 可传入 symbol 参数（默认 "39693"）
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/article_rlab_rv",
        tag = "ak",
        params(
            RlabRvQuery
        ),
        responses(
            (status = 200, description = "成功获取 Risk Lab 实际波动率数据", body = ApiResponse<Vec<VolatilityItem>>)
        )
    )
)]
pub async fn get_article_rlab_rv(query: web::Query<RlabRvQuery>) -> Result<HttpResponse> {
    match ak::article_rlab_rv(query.symbol.clone()).await {
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
            .route("/article_epu_index", web::get().to(get_article_epu_index))
            .route("/fred_md", web::get().to(get_fred_md))
            .route("/fred_qd", web::get().to(get_fred_qd))
            .route("/article_oman_rv", web::get().to(get_article_oman_rv))
            .route(
                "/article_oman_rv_short",
                web::get().to(get_article_oman_rv_short),
            )
            .route("/article_rlab_rv", web::get().to(get_article_rlab_rv))
            .configure(bank::config)
            .configure(bond::config)
            .configure(currency::config)
            .configure(cal::config)
            .configure(interest_rate::config)
            .configure(crypto::config)
            .configure(energy::config)
            .configure(forex::config)
            .configure(fortune::config)
            .configure(fund::config)
            .configure(futures_settle::config)
            .configure(macro_cnbs::config)
            .configure(migration::config)
            .configure(macro_data::config),
    );
}
