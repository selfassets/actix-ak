//! 宏观经济数据 HTTP 处理器

use crate::models::ApiResponse;
use crate::services::ak::macro_data;
use actix_web::{web, HttpResponse, Result};

/// 获取中国 GDP 年率/季率数据
///
/// GET /api/v1/ak/macro/china_gdp
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/china_gdp",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取中国 GDP 数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_china_gdp() -> Result<HttpResponse> {
    match macro_data::get_macro_china_gdp().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取中国 CPI 居民消费价格指数数据
///
/// GET /api/v1/ak/macro/china_cpi
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/china_cpi",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取中国 CPI 数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_china_cpi() -> Result<HttpResponse> {
    match macro_data::get_macro_china_cpi().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取中国 PPI 工业生产者出厂价格指数数据
///
/// GET /api/v1/ak/macro/china_ppi
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/china_ppi",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取中国 PPI 数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_china_ppi() -> Result<HttpResponse> {
    match macro_data::get_macro_china_ppi().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取中国官方 PMI 采购经理人指数数据
///
/// GET /api/v1/ak/macro/china_pmi
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/china_pmi",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取中国 PMI 数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_china_pmi() -> Result<HttpResponse> {
    match macro_data::get_macro_china_pmi().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取中国社会融资规模增量及数据
///
/// GET /api/v1/ak/macro/china_shrzgm
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/china_shrzgm",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取中国社会融资规模数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_china_shrzgm() -> Result<HttpResponse> {
    match macro_data::get_macro_china_shrzgm().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取中国 M2 货币供应量数据
///
/// GET /api/v1/ak/macro/china_m2
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/china_m2",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取中国 M2 货币供应量", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_china_m2() -> Result<HttpResponse> {
    match macro_data::get_macro_china_m2().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取中国 LPR 贷款市场报价利率历史数据
///
/// GET /api/v1/ak/macro/china_lpr
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/china_lpr",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取中国 LPR 利率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_china_lpr() -> Result<HttpResponse> {
    match macro_data::get_macro_china_lpr().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取美国非农就业人口变动数据
///
/// GET /api/v1/ak/macro/usa_non_farm
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/usa_non_farm",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取美国非农数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_usa_non_farm() -> Result<HttpResponse> {
    match macro_data::get_macro_usa_non_farm().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取美国失业率数据
///
/// GET /api/v1/ak/macro/usa_unemployment
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/usa_unemployment",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取美国失业率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_usa_unemployment() -> Result<HttpResponse> {
    match macro_data::get_macro_usa_unemployment().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取美国 CPI 消费者物价指数数据
///
/// GET /api/v1/ak/macro/usa_cpi
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/usa_cpi",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取美国 CPI 数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_usa_cpi() -> Result<HttpResponse> {
    match macro_data::get_macro_usa_cpi().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取美国 GDP 增长数据
///
/// GET /api/v1/ak/macro/usa_gdp
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/usa_gdp",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取美国 GDP 数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_usa_gdp() -> Result<HttpResponse> {
    match macro_data::get_macro_usa_gdp().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取中国央行基准利率决议数据
///
/// GET /api/v1/ak/macro/bank_china_interest_rate
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/bank_china_interest_rate",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取中国央行利率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_bank_china_interest_rate() -> Result<HttpResponse> {
    match macro_data::get_macro_bank_china_interest_rate().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取美联储基准利率决议数据
///
/// GET /api/v1/ak/macro/bank_usa_interest_rate
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/bank_usa_interest_rate",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取美联储利率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_bank_usa_interest_rate() -> Result<HttpResponse> {
    match macro_data::get_macro_bank_usa_interest_rate().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取欧洲央行基准利率决议数据
///
/// GET /api/v1/ak/macro/bank_euro_interest_rate
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/bank_euro_interest_rate",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取欧洲央行利率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_bank_euro_interest_rate() -> Result<HttpResponse> {
    match macro_data::get_macro_bank_euro_interest_rate().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取日本央行基准利率决议数据
///
/// GET /api/v1/ak/macro/bank_japan_interest_rate
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/bank_japan_interest_rate",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取日本央行利率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_bank_japan_interest_rate() -> Result<HttpResponse> {
    match macro_data::get_macro_bank_japan_interest_rate().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 配置宏观数据路由
///
/// 挂载路径：/macro
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/macro")
            .route("/china_gdp", web::get().to(get_macro_china_gdp))
            .route("/china_cpi", web::get().to(get_macro_china_cpi))
            .route("/china_ppi", web::get().to(get_macro_china_ppi))
            .route("/china_pmi", web::get().to(get_macro_china_pmi))
            .route("/china_shrzgm", web::get().to(get_macro_china_shrzgm))
            .route("/china_m2", web::get().to(get_macro_china_m2))
            .route("/china_lpr", web::get().to(get_macro_china_lpr))
            .route("/usa_non_farm", web::get().to(get_macro_usa_non_farm))
            .route(
                "/usa_unemployment",
                web::get().to(get_macro_usa_unemployment),
            )
            .route("/usa_cpi", web::get().to(get_macro_usa_cpi))
            .route("/usa_gdp", web::get().to(get_macro_usa_gdp))
            .route(
                "/bank_china_interest_rate",
                web::get().to(get_macro_bank_china_interest_rate),
            )
            .route(
                "/bank_usa_interest_rate",
                web::get().to(get_macro_bank_usa_interest_rate),
            )
            .route(
                "/bank_euro_interest_rate",
                web::get().to(get_macro_bank_euro_interest_rate),
            )
            .route(
                "/bank_japan_interest_rate",
                web::get().to(get_macro_bank_japan_interest_rate),
            ),
    );
}
