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

/// 获取新西兰联储基准利率决议数据
///
/// GET /api/v1/ak/macro/bank_newzealand_interest_rate
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/bank_newzealand_interest_rate",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取新西兰央行利率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_bank_newzealand_interest_rate() -> Result<HttpResponse> {
    match macro_data::get_macro_bank_newzealand_interest_rate().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取瑞士央行基准利率决议数据
///
/// GET /api/v1/ak/macro/bank_switzerland_interest_rate
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/bank_switzerland_interest_rate",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取瑞士央行利率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_bank_switzerland_interest_rate() -> Result<HttpResponse> {
    match macro_data::get_macro_bank_switzerland_interest_rate().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取英国央行基准利率决议数据
///
/// GET /api/v1/ak/macro/bank_english_interest_rate
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/bank_english_interest_rate",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取英国央行利率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_bank_english_interest_rate() -> Result<HttpResponse> {
    match macro_data::get_macro_bank_english_interest_rate().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取澳洲联储基准利率决议数据
///
/// GET /api/v1/ak/macro/bank_australia_interest_rate
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/bank_australia_interest_rate",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取澳洲联储利率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_bank_australia_interest_rate() -> Result<HttpResponse> {
    match macro_data::get_macro_bank_australia_interest_rate().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取俄罗斯央行基准利率决议数据
///
/// GET /api/v1/ak/macro/bank_russia_interest_rate
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/bank_russia_interest_rate",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取俄罗斯央行利率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_bank_russia_interest_rate() -> Result<HttpResponse> {
    match macro_data::get_macro_bank_russia_interest_rate().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取印度央行基准利率决议数据
///
/// GET /api/v1/ak/macro/bank_india_interest_rate
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/bank_india_interest_rate",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取印度央行利率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_bank_india_interest_rate() -> Result<HttpResponse> {
    match macro_data::get_macro_bank_india_interest_rate().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取巴西央行基准利率决议数据
///
/// GET /api/v1/ak/macro/bank_brazil_interest_rate
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/bank_brazil_interest_rate",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取巴西央行利率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_bank_brazil_interest_rate() -> Result<HttpResponse> {
    match macro_data::get_macro_bank_brazil_interest_rate().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取中国出口年率数据
///
/// GET /api/v1/ak/macro/china_exports_yoy
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/china_exports_yoy",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取中国出口年率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_china_exports_yoy() -> Result<HttpResponse> {
    match macro_data::get_macro_china_exports_yoy().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取中国进口年率数据
///
/// GET /api/v1/ak/macro/china_imports_yoy
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/china_imports_yoy",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取中国进口年率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_china_imports_yoy() -> Result<HttpResponse> {
    match macro_data::get_macro_china_imports_yoy().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取中国贸易帐数据
///
/// GET /api/v1/ak/macro/china_trade_balance
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/china_trade_balance",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取中国贸易帐数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_china_trade_balance() -> Result<HttpResponse> {
    match macro_data::get_macro_china_trade_balance().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取中国工业增加值年率数据
///
/// GET /api/v1/ak/macro/china_industrial_production_yoy
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/china_industrial_production_yoy",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取中国工业增加值数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_china_industrial_production_yoy() -> Result<HttpResponse> {
    match macro_data::get_macro_china_industrial_production_yoy().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取中国财新制造业 PMI 数据
///
/// GET /api/v1/ak/macro/china_cx_pmi_yearly
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/china_cx_pmi_yearly",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取中国财新 PMI 数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_china_cx_pmi_yearly() -> Result<HttpResponse> {
    match macro_data::get_macro_china_cx_pmi_yearly().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取中国财新服务业 PMI 数据
///
/// GET /api/v1/ak/macro/china_cx_services_pmi_yearly
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/china_cx_services_pmi_yearly",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取中国财新服务业 PMI 数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_china_cx_services_pmi_yearly() -> Result<HttpResponse> {
    match macro_data::get_macro_china_cx_services_pmi_yearly().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取中国官方非制造业 PMI 数据
///
/// GET /api/v1/ak/macro/china_non_man_pmi
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/china_non_man_pmi",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取中国官方非制造业 PMI 数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_china_non_man_pmi() -> Result<HttpResponse> {
    match macro_data::get_macro_china_non_man_pmi().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取中国外汇储备数据
///
/// GET /api/v1/ak/macro/china_fx_reserves_yearly
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/china_fx_reserves_yearly",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取中国外汇储备数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_china_fx_reserves_yearly() -> Result<HttpResponse> {
    match macro_data::get_macro_china_fx_reserves_yearly().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取美国 ADP 就业数据
///
/// GET /api/v1/ak/macro/usa_adp_employment
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/usa_adp_employment",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取美国 ADP 就业数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_usa_adp_employment() -> Result<HttpResponse> {
    match macro_data::get_macro_usa_adp_employment().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取美国初请失业金数据
///
/// GET /api/v1/ak/macro/usa_initial_jobless
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/usa_initial_jobless",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取美国初请失业金数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_usa_initial_jobless() -> Result<HttpResponse> {
    match macro_data::get_macro_usa_initial_jobless().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取美国 PPI 生产者物价指数数据
///
/// GET /api/v1/ak/macro/usa_ppi
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/usa_ppi",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取美国 PPI 数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_usa_ppi() -> Result<HttpResponse> {
    match macro_data::get_macro_usa_ppi().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取美国 ISM 制造业 PMI 数据
///
/// GET /api/v1/ak/macro/usa_ism_pmi
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/usa_ism_pmi",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取美国 ISM PMI 数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_usa_ism_pmi() -> Result<HttpResponse> {
    match macro_data::get_macro_usa_ism_pmi().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取美国零售销售月率数据
///
/// GET /api/v1/ak/macro/usa_retail_sales
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/usa_retail_sales",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取美国零售销售数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_usa_retail_sales() -> Result<HttpResponse> {
    match macro_data::get_macro_usa_retail_sales().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取美国工业产出月率数据
///
/// GET /api/v1/ak/macro/usa_industrial_production
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/usa_industrial_production",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取美国工业产出数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_usa_industrial_production() -> Result<HttpResponse> {
    match macro_data::get_macro_usa_industrial_production().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取欧元区 GDP 季率数据
///
/// GET /api/v1/ak/macro/euro_gdp_yoy
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/euro_gdp_yoy",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取欧元区 GDP 数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_euro_gdp_yoy() -> Result<HttpResponse> {
    match macro_data::get_macro_euro_gdp_yoy().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取欧元区 CPI 年率数据
///
/// GET /api/v1/ak/macro/euro_cpi_yoy
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/euro_cpi_yoy",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取欧元区 CPI 数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_euro_cpi_yoy() -> Result<HttpResponse> {
    match macro_data::get_macro_euro_cpi_yoy().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取德国 IFO 商业景气指数数据
///
/// GET /api/v1/ak/macro/germany_ifo
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/germany_ifo",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取德国 IFO 数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_germany_ifo() -> Result<HttpResponse> {
    match macro_data::get_macro_germany_ifo().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取英国 GDP 季率数据
///
/// GET /api/v1/ak/macro/uk_gdp_quarterly
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/uk_gdp_quarterly",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取英国 GDP 数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_uk_gdp_quarterly() -> Result<HttpResponse> {
    match macro_data::get_macro_uk_gdp_quarterly().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取澳大利亚失业率数据
///
/// GET /api/v1/ak/macro/australia_unemployment_rate
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/australia_unemployment_rate",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取澳大利亚失业率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_australia_unemployment_rate() -> Result<HttpResponse> {
    match macro_data::get_macro_australia_unemployment_rate().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取黄金 ETF 持仓数据
///
/// GET /api/v1/ak/macro/cons_gold
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/cons_gold",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取黄金 ETF 持仓数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_cons_gold() -> Result<HttpResponse> {
    match macro_data::get_macro_cons_gold().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取白银 ETF 持仓数据
///
/// GET /api/v1/ak/macro/cons_silver
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/cons_silver",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取白银 ETF 持仓数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_cons_silver() -> Result<HttpResponse> {
    match macro_data::get_macro_cons_silver().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取 OPEC 月度原油产量数据
///
/// GET /api/v1/ak/macro/cons_opec_month
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/cons_opec_month",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取 OPEC 原油产量数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_cons_opec_month() -> Result<HttpResponse> {
    match macro_data::get_macro_cons_opec_month().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取美国 CPI 月率数据
///
/// GET /api/v1/ak/macro/usa_cpi_monthly
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/usa_cpi_monthly",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取美国 CPI 月率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_usa_cpi_monthly() -> Result<HttpResponse> {
    match macro_data::get_macro_usa_cpi_monthly().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取美国核心 CPI 月率数据
///
/// GET /api/v1/ak/macro/usa_core_cpi_monthly
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/usa_core_cpi_monthly",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取美国核心 CPI 月率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_usa_core_cpi_monthly() -> Result<HttpResponse> {
    match macro_data::get_macro_usa_core_cpi_monthly().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取美国核心 PCE 物价指数数据
///
/// GET /api/v1/ak/macro/usa_core_pce_price
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/usa_core_pce_price",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取美国核心 PCE 数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_usa_core_pce_price() -> Result<HttpResponse> {
    match macro_data::get_macro_usa_core_pce_price().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取美国贸易帐数据
///
/// GET /api/v1/ak/macro/usa_trade_balance
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/usa_trade_balance",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取美国贸易帐数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_usa_trade_balance() -> Result<HttpResponse> {
    match macro_data::get_macro_usa_trade_balance().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取美国 API 原油库存数据
///
/// GET /api/v1/ak/macro/usa_api_crude_stock
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/usa_api_crude_stock",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取美国 API 原油库存数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_usa_api_crude_stock() -> Result<HttpResponse> {
    match macro_data::get_macro_usa_api_crude_stock().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取美国 Markit 制造业 PMI 数据
///
/// GET /api/v1/ak/macro/usa_pmi
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/usa_pmi",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取美国 Markit PMI 数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_usa_pmi() -> Result<HttpResponse> {
    match macro_data::get_macro_usa_pmi().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取美国 ISM 非制造业 PMI 数据
///
/// GET /api/v1/ak/macro/usa_ism_non_pmi
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/usa_ism_non_pmi",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取美国 ISM 非制造业 PMI 数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_usa_ism_non_pmi() -> Result<HttpResponse> {
    match macro_data::get_macro_usa_ism_non_pmi().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取美国新屋开工总数年化数据
///
/// GET /api/v1/ak/macro/usa_house_starts
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/usa_house_starts",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取美国新屋开工数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_usa_house_starts() -> Result<HttpResponse> {
    match macro_data::get_macro_usa_house_starts().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取美国新屋销售总数年化数据
///
/// GET /api/v1/ak/macro/usa_new_home_sales
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/usa_new_home_sales",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取美国新屋销售数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_usa_new_home_sales() -> Result<HttpResponse> {
    match macro_data::get_macro_usa_new_home_sales().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取美国营建许可总数数据
///
/// GET /api/v1/ak/macro/usa_building_permits
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/usa_building_permits",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取美国营建许可数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_usa_building_permits() -> Result<HttpResponse> {
    match macro_data::get_macro_usa_building_permits().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取美国谘商会消费者信心指数数据
///
/// GET /api/v1/ak/macro/usa_cb_consumer_confidence
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/usa_cb_consumer_confidence",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取美国 CB 消费者信心数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_usa_cb_consumer_confidence() -> Result<HttpResponse> {
    match macro_data::get_macro_usa_cb_consumer_confidence().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取美国密歇根大学消费者信心指数数据
///
/// GET /api/v1/ak/macro/usa_michigan_consumer_sentiment
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/usa_michigan_consumer_sentiment",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取美国密歇根消费者信心数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_usa_michigan_consumer_sentiment() -> Result<HttpResponse> {
    match macro_data::get_macro_usa_michigan_consumer_sentiment().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取欧元区工业产出月率数据
///
/// GET /api/v1/ak/macro/euro_industrial_production_mom
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/euro_industrial_production_mom",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取欧元区工业产出数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_euro_industrial_production_mom() -> Result<HttpResponse> {
    match macro_data::get_macro_euro_industrial_production_mom().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取欧元区制造业 PMI 数据
///
/// GET /api/v1/ak/macro/euro_manufacturing_pmi
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/euro_manufacturing_pmi",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取欧元区制造业 PMI 数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_euro_manufacturing_pmi() -> Result<HttpResponse> {
    match macro_data::get_macro_euro_manufacturing_pmi().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取欧元区服务业 PMI 数据
///
/// GET /api/v1/ak/macro/euro_services_pmi
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/euro_services_pmi",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取欧元区服务业 PMI 数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_euro_services_pmi() -> Result<HttpResponse> {
    match macro_data::get_macro_euro_services_pmi().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取欧元区 ZEW 经济景气指数数据
///
/// GET /api/v1/ak/macro/euro_zew_economic_sentiment
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/euro_zew_economic_sentiment",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取欧元区 ZEW 经济景气指数", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_euro_zew_economic_sentiment() -> Result<HttpResponse> {
    match macro_data::get_macro_euro_zew_economic_sentiment().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取德国 CPI 月率数据
///
/// GET /api/v1/ak/macro/germany_cpi_monthly
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/germany_cpi_monthly",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取德国 CPI 月率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_germany_cpi_monthly() -> Result<HttpResponse> {
    match macro_data::get_macro_germany_cpi_monthly().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取德国 GDP 季率数据
///
/// GET /api/v1/ak/macro/germany_gdp
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/germany_gdp",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取德国 GDP 季率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_germany_gdp() -> Result<HttpResponse> {
    match macro_data::get_macro_germany_gdp().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取英国 CPI 月率数据
///
/// GET /api/v1/ak/macro/uk_cpi_monthly
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/uk_cpi_monthly",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取英国 CPI 月率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_uk_cpi_monthly() -> Result<HttpResponse> {
    match macro_data::get_macro_uk_cpi_monthly().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取英国失业率数据
///
/// GET /api/v1/ak/macro/uk_unemployment_rate
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/uk_unemployment_rate",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取英国失业率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_uk_unemployment_rate() -> Result<HttpResponse> {
    match macro_data::get_macro_uk_unemployment_rate().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取澳大利亚零售销售月率数据
///
/// GET /api/v1/ak/macro/australia_retail_rate_monthly
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/australia_retail_rate_monthly",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取澳大利亚零售销售数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_australia_retail_rate_monthly() -> Result<HttpResponse> {
    match macro_data::get_macro_australia_retail_rate_monthly().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取澳大利亚 PPI 季率数据
///
/// GET /api/v1/ak/macro/australia_ppi_quarterly
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/australia_ppi_quarterly",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取澳大利亚 PPI 数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_australia_ppi_quarterly() -> Result<HttpResponse> {
    match macro_data::get_macro_australia_ppi_quarterly().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取加拿大失业率数据
///
/// GET /api/v1/ak/macro/canada_unemployment_rate
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/canada_unemployment_rate",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取加拿大失业率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_canada_unemployment_rate() -> Result<HttpResponse> {
    match macro_data::get_macro_canada_unemployment_rate().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取日本 CPI 年率数据
///
/// GET /api/v1/ak/macro/japan_cpi_yearly
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/japan_cpi_yearly",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取日本 CPI 年率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_japan_cpi_yearly() -> Result<HttpResponse> {
    match macro_data::get_macro_japan_cpi_yearly().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取瑞士 CPI 年率数据
///
/// GET /api/v1/ak/macro/swiss_cpi_yearly
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/swiss_cpi_yearly",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取瑞士 CPI 年率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_swiss_cpi_yearly() -> Result<HttpResponse> {
    match macro_data::get_macro_swiss_cpi_yearly().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取瑞士 SVME 采购经理人指数数据
///
/// GET /api/v1/ak/macro/swiss_svme
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/swiss_svme",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取瑞士 SVME PMI 数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_swiss_svme() -> Result<HttpResponse> {
    match macro_data::get_macro_swiss_svme().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取中国城镇调查失业率数据
///
/// GET /api/v1/ak/macro/china_urban_unemployment
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/china_urban_unemployment",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取中国城镇失业率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_china_urban_unemployment() -> Result<HttpResponse> {
    match macro_data::get_macro_china_urban_unemployment().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取中国社会消费品零售总额年率数据
///
/// GET /api/v1/ak/macro/china_consumer_goods_retail
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/china_consumer_goods_retail",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取中国社零消费数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_china_consumer_goods_retail() -> Result<HttpResponse> {
    match macro_data::get_macro_china_consumer_goods_retail().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取中国 CPI 月率数据
///
/// GET /api/v1/ak/macro/china_cpi_monthly
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/china_cpi_monthly",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取中国 CPI 月率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_china_cpi_monthly() -> Result<HttpResponse> {
    match macro_data::get_macro_china_cpi_monthly().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取中国 PPI 年率数据
///
/// GET /api/v1/ak/macro/china_ppi_yearly
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/china_ppi_yearly",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取中国 PPI 年率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_china_ppi_yearly() -> Result<HttpResponse> {
    match macro_data::get_macro_china_ppi_yearly().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取中国官方制造业 PMI 年度数据
///
/// GET /api/v1/ak/macro/china_pmi_yearly
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/china_pmi_yearly",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取中国官方 PMI 年度数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_china_pmi_yearly() -> Result<HttpResponse> {
    match macro_data::get_macro_china_pmi_yearly().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取中国 M2 货币供应年率数据
///
/// GET /api/v1/ak/macro/china_m2_yearly
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/china_m2_yearly",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取中国 M2 年率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_china_m2_yearly() -> Result<HttpResponse> {
    match macro_data::get_macro_china_m2_yearly().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取美国成屋签约销售指数数据
///
/// GET /api/v1/ak/macro/usa_pending_home_sales
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/usa_pending_home_sales",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取美国成屋签约销售数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_usa_pending_home_sales() -> Result<HttpResponse> {
    match macro_data::get_macro_usa_pending_home_sales().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取美国成屋销售总数年化数据
///
/// GET /api/v1/ak/macro/usa_exist_home_sales
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/usa_exist_home_sales",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取美国成屋销售数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_usa_exist_home_sales() -> Result<HttpResponse> {
    match macro_data::get_macro_usa_exist_home_sales().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取美国商业库存月率数据
///
/// GET /api/v1/ak/macro/usa_business_inventories
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/usa_business_inventories",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取美国商业库存数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_usa_business_inventories() -> Result<HttpResponse> {
    match macro_data::get_macro_usa_business_inventories().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取美国工厂订单月率数据
///
/// GET /api/v1/ak/macro/usa_factory_orders
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/usa_factory_orders",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取美国工厂订单数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_usa_factory_orders() -> Result<HttpResponse> {
    match macro_data::get_macro_usa_factory_orders().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取加拿大 CPI 年率数据
///
/// GET /api/v1/ak/macro/canada_cpi_yearly
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/canada_cpi_yearly",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取加拿大 CPI 年率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_canada_cpi_yearly() -> Result<HttpResponse> {
    match macro_data::get_macro_canada_cpi_yearly().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取澳大利亚 CPI 季率数据
///
/// GET /api/v1/ak/macro/australia_cpi_quarterly
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/australia_cpi_quarterly",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取澳大利亚 CPI 季率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_australia_cpi_quarterly() -> Result<HttpResponse> {
    match macro_data::get_macro_australia_cpi_quarterly().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取英国贸易帐数据
///
/// GET /api/v1/ak/macro/uk_trade
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/uk_trade",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取英国贸易帐数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_uk_trade() -> Result<HttpResponse> {
    match macro_data::get_macro_uk_trade().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取日本央行核心 CPI 年率数据
///
/// GET /api/v1/ak/macro/japan_core_cpi_yearly
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/japan_core_cpi_yearly",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取日本核心 CPI 数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_japan_core_cpi_yearly() -> Result<HttpResponse> {
    match macro_data::get_macro_japan_core_cpi_yearly().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取同花顺股票筹资数据
///
/// GET /api/v1/ak/macro/stock_finance
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/stock_finance",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取同花顺股票筹资数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_stock_finance() -> Result<HttpResponse> {
    match macro_data::get_macro_stock_finance().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取香港 CPI 年率数据
///
/// GET /api/v1/ak/macro/china_hk_cpi
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/china_hk_cpi",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取香港 CPI 年率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_china_hk_cpi() -> Result<HttpResponse> {
    match macro_data::get_macro_china_hk_cpi().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取香港失业率数据
///
/// GET /api/v1/ak/macro/china_hk_rate_of_unemployment
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/china_hk_rate_of_unemployment",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取香港失业率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_china_hk_rate_of_unemployment() -> Result<HttpResponse> {
    match macro_data::get_macro_china_hk_rate_of_unemployment().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取香港 GDP 年率数据
///
/// GET /api/v1/ak/macro/china_hk_gbp
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/china_hk_gbp",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取香港 GDP 年率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_china_hk_gbp() -> Result<HttpResponse> {
    match macro_data::get_macro_china_hk_gbp().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取香港贸易帐数据
///
/// GET /api/v1/ak/macro/china_hk_trade_diff_ratio
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/china_hk_trade_diff_ratio",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取香港贸易帐数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_china_hk_trade_diff_ratio() -> Result<HttpResponse> {
    match macro_data::get_macro_china_hk_trade_diff_ratio().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取欧洲存款机制利率数据
///
/// GET /api/v1/ak/macro/euro_deposit_rate
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/euro_deposit_rate",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取欧洲存款机制利率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_euro_deposit_rate() -> Result<HttpResponse> {
    match macro_data::get_macro_euro_deposit_rate().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取欧洲边际贷款利率数据
///
/// GET /api/v1/ak/macro/euro_marginal_lending_rate
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/euro_marginal_lending_rate",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取欧洲边际贷款利率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_euro_marginal_lending_rate() -> Result<HttpResponse> {
    match macro_data::get_macro_euro_marginal_lending_rate().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取加拿大央行利率决议数据
///
/// GET /api/v1/ak/macro/bank_canada_interest_rate
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/bank_canada_interest_rate",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取加拿大央行利率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_bank_canada_interest_rate() -> Result<HttpResponse> {
    match macro_data::get_macro_bank_canada_interest_rate().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取美国耐用品订单月率数据
///
/// GET /api/v1/ak/macro/usa_durable_goods_orders
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/usa_durable_goods_orders",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取美国耐用品订单数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_usa_durable_goods_orders() -> Result<HttpResponse> {
    match macro_data::get_macro_usa_durable_goods_orders().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取美国个人支出月率数据
///
/// GET /api/v1/ak/macro/usa_personal_spending
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/usa_personal_spending",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取美国个人支出数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_usa_personal_spending() -> Result<HttpResponse> {
    match macro_data::get_macro_usa_personal_spending().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取中国城镇固定资产投资数据(东方财富)
///
/// GET /api/v1/ak/macro/china_gdzctz
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/china_gdzctz",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取中国固定资产投资数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_china_gdzctz() -> Result<HttpResponse> {
    match macro_data::get_macro_china_gdzctz().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取中国海关进出口状况数据
///
/// GET /api/v1/ak/macro/china_hgjck
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/china_hgjck",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取中国海关进出口数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_china_hgjck() -> Result<HttpResponse> {
    match macro_data::get_macro_china_hgjck().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取中国财政收入数据
///
/// GET /api/v1/ak/macro/china_czsr
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/china_czsr",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取中国财政收入数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_china_czsr() -> Result<HttpResponse> {
    match macro_data::get_macro_china_czsr().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取中国外汇信贷数据
///
/// GET /api/v1/ak/macro/china_whxd
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/china_whxd",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取中国外汇信贷数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_china_whxd() -> Result<HttpResponse> {
    match macro_data::get_macro_china_whxd().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取中国消费者信心指数数据
///
/// GET /api/v1/ak/macro/china_xfzxx
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/china_xfzxx",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取中国消费者信心数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_china_xfzxx() -> Result<HttpResponse> {
    match macro_data::get_macro_china_xfzxx().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取中国存款准备金率数据
///
/// GET /api/v1/ak/macro/china_reserve_requirement_ratio
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/china_reserve_requirement_ratio",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取中国存款准备金数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_china_reserve_requirement_ratio() -> Result<HttpResponse> {
    match macro_data::get_macro_china_reserve_requirement_ratio().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取加拿大零售销售月率数据
///
/// GET /api/v1/ak/macro/canada_retail_rate_monthly
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/canada_retail_rate_monthly",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取加拿大零售销售月率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_canada_retail_rate_monthly() -> Result<HttpResponse> {
    match macro_data::get_macro_canada_retail_rate_monthly().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取德国零售销售月率数据
///
/// GET /api/v1/ak/macro/germany_retail_sale_monthly
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/germany_retail_sale_monthly",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取德国零售销售月率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_germany_retail_sale_monthly() -> Result<HttpResponse> {
    match macro_data::get_macro_germany_retail_sale_monthly().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取英国零售销售月率数据
///
/// GET /api/v1/ak/macro/uk_retail_monthly
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/uk_retail_monthly",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取英国零售销售月率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_uk_retail_monthly() -> Result<HttpResponse> {
    match macro_data::get_macro_uk_retail_monthly().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取瑞士贸易帐数据
///
/// GET /api/v1/ak/macro/swiss_trade
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/swiss_trade",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取瑞士贸易帐数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_swiss_trade() -> Result<HttpResponse> {
    match macro_data::get_macro_swiss_trade().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取德国经调贸易帐数据
///
/// GET /api/v1/ak/macro/germany_trade_adjusted
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/germany_trade_adjusted",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取德国经调贸易帐数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_germany_trade_adjusted() -> Result<HttpResponse> {
    match macro_data::get_macro_germany_trade_adjusted().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取加拿大贸易帐数据
///
/// GET /api/v1/ak/macro/canada_trade
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/canada_trade",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取加拿大贸易帐数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_canada_trade() -> Result<HttpResponse> {
    match macro_data::get_macro_canada_trade().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取瑞士 GBD 年率数据
///
/// GET /api/v1/ak/macro/swiss_gbd_yearly
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/swiss_gbd_yearly",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取瑞士 GBD 年率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_swiss_gbd_yearly() -> Result<HttpResponse> {
    match macro_data::get_macro_swiss_gbd_yearly().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取英国 Halifax 房价指数月率数据
///
/// GET /api/v1/ak/macro/uk_halifax_monthly
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/uk_halifax_monthly",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取英国 Halifax 房价指数月率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_uk_halifax_monthly() -> Result<HttpResponse> {
    match macro_data::get_macro_uk_halifax_monthly().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取英国 Halifax 房价指数年率数据
///
/// GET /api/v1/ak/macro/uk_halifax_yearly
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/uk_halifax_yearly",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取英国 Halifax 房价指数年率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_uk_halifax_yearly() -> Result<HttpResponse> {
    match macro_data::get_macro_uk_halifax_yearly().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取英国 Rightmove 房价指数月率数据
///
/// GET /api/v1/ak/macro/uk_rightmove_monthly
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/uk_rightmove_monthly",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取英国 Rightmove 房价指数月率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_uk_rightmove_monthly() -> Result<HttpResponse> {
    match macro_data::get_macro_uk_rightmove_monthly().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取德国零售销售年率数据
///
/// GET /api/v1/ak/macro/germany_retail_sale_yearly
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/germany_retail_sale_yearly",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取德国零售销售年率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_germany_retail_sale_yearly() -> Result<HttpResponse> {
    match macro_data::get_macro_germany_retail_sale_yearly().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取加拿大新屋指数数据
///
/// GET /api/v1/ak/macro/canada_new_house_rate
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/canada_new_house_rate",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取加拿大新屋指数数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_canada_new_house_rate() -> Result<HttpResponse> {
    match macro_data::get_macro_canada_new_house_rate().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取加拿大央行其他小类利率数据
///
/// GET /api/v1/ak/macro/canada_bank_rate
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/canada_bank_rate",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取加拿大其他央行利率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_canada_bank_rate() -> Result<HttpResponse> {
    match macro_data::get_macro_canada_bank_rate().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取香港楼宇买卖交易件数数据
///
/// GET /api/v1/ak/macro/china_hk_building_volume
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/china_hk_building_volume",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取香港楼宇买卖交易件数数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_china_hk_building_volume() -> Result<HttpResponse> {
    match macro_data::get_macro_china_hk_building_volume().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取香港楼宇买卖交易金额数据
///
/// GET /api/v1/ak/macro/china_hk_building_amount
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/china_hk_building_amount",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取香港楼宇买卖交易金额数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_china_hk_building_amount() -> Result<HttpResponse> {
    match macro_data::get_macro_china_hk_building_amount().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取加拿大核心 CPI 年率数据
///
/// GET /api/v1/ak/macro/canada_core_cpi_yearly
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/canada_core_cpi_yearly",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取加拿大核心 CPI 年率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_canada_core_cpi_yearly() -> Result<HttpResponse> {
    match macro_data::get_macro_canada_core_cpi_yearly().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取加拿大核心 CPI 月率数据
///
/// GET /api/v1/ak/macro/canada_core_cpi_monthly
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/canada_core_cpi_monthly",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取加拿大核心 CPI 月率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_canada_core_cpi_monthly() -> Result<HttpResponse> {
    match macro_data::get_macro_canada_core_cpi_monthly().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取加拿大 CPI 月率数据
///
/// GET /api/v1/ak/macro/canada_cpi_monthly
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/canada_cpi_monthly",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取加拿大 CPI 月率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_canada_cpi_monthly() -> Result<HttpResponse> {
    match macro_data::get_macro_canada_cpi_monthly().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取英国核心 CPI 年率数据
///
/// GET /api/v1/ak/macro/uk_core_cpi_yearly
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/uk_core_cpi_yearly",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取英国核心 CPI 年率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_uk_core_cpi_yearly() -> Result<HttpResponse> {
    match macro_data::get_macro_uk_core_cpi_yearly().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取英国核心 CPI 月率数据
///
/// GET /api/v1/ak/macro/uk_core_cpi_monthly
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/uk_core_cpi_monthly",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取英国核心 CPI 月率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_uk_core_cpi_monthly() -> Result<HttpResponse> {
    match macro_data::get_macro_uk_core_cpi_monthly().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取英国 CPI 年率数据
///
/// GET /api/v1/ak/macro/uk_cpi_yearly
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/uk_cpi_yearly",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取英国 CPI 年率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_uk_cpi_yearly() -> Result<HttpResponse> {
    match macro_data::get_macro_uk_cpi_yearly().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取德国 GDP 年率数据
///
/// GET /api/v1/ak/macro/germany_gdp_yearly
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/germany_gdp_yearly",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取德国 GDP 年率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_germany_gdp_yearly() -> Result<HttpResponse> {
    match macro_data::get_macro_germany_gdp_yearly().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取贝克休斯美国钻井总数数据
///
/// GET /api/v1/ak/macro/usa_rig_count
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/usa_rig_count",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取美国钻井总数数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_usa_rig_count() -> Result<HttpResponse> {
    match macro_data::get_macro_usa_rig_count().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取美国 EIA 原油库存变化率数据
///
/// GET /api/v1/ak/macro/usa_eia_crude_rate
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/usa_eia_crude_rate",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取美国 EIA 原油变化率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_usa_eia_crude_rate() -> Result<HttpResponse> {
    match macro_data::get_macro_usa_eia_crude_rate().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取同花顺新增人民币贷款数据
///
/// GET /api/v1/ak/macro/rmb_loan
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/rmb_loan",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取新增人民币贷款数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_rmb_loan() -> Result<HttpResponse> {
    match macro_data::get_macro_rmb_loan().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取同花顺人民币存款余额数据
///
/// GET /api/v1/ak/macro/rmb_deposit
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/rmb_deposit",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取人民币存款余额数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_rmb_deposit() -> Result<HttpResponse> {
    match macro_data::get_macro_rmb_deposit().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取华尔街见闻宏观经济日历
///
/// GET /api/v1/ak/macro/info_ws
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/info_ws",
        tag = "macro",
        params(
            ("date" = Option<String>, Query, description = "要查询的日期，如 20240514 等")
        ),
        responses(
            (status = 200, description = "成功获取宏观经济日历数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_info_ws(
    query: web::Query<crate::models::ak::energy::EnergyOilQuery>,
) -> Result<HttpResponse> {
    let date = query.date.clone().unwrap_or_else(|| "20240514".to_string());
    match macro_data::get_macro_info_ws(&date).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取英国 Rightmove 房价指数年率数据
///
/// GET /api/v1/ak/macro/uk_rightmove_yearly
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/macro/uk_rightmove_yearly",
        tag = "macro",
        responses(
            (status = 200, description = "成功获取英国 Rightmove 房价指数年率数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_macro_uk_rightmove_yearly() -> Result<HttpResponse> {
    match macro_data::get_macro_uk_rightmove_yearly().await {
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
            )
            .route(
                "/bank_newzealand_interest_rate",
                web::get().to(get_macro_bank_newzealand_interest_rate),
            )
            .route(
                "/bank_switzerland_interest_rate",
                web::get().to(get_macro_bank_switzerland_interest_rate),
            )
            .route(
                "/bank_english_interest_rate",
                web::get().to(get_macro_bank_english_interest_rate),
            )
            .route(
                "/bank_australia_interest_rate",
                web::get().to(get_macro_bank_australia_interest_rate),
            )
            .route(
                "/bank_russia_interest_rate",
                web::get().to(get_macro_bank_russia_interest_rate),
            )
            .route(
                "/bank_india_interest_rate",
                web::get().to(get_macro_bank_india_interest_rate),
            )
            .route(
                "/bank_brazil_interest_rate",
                web::get().to(get_macro_bank_brazil_interest_rate),
            )
            .route(
                "/china_exports_yoy",
                web::get().to(get_macro_china_exports_yoy),
            )
            .route(
                "/china_imports_yoy",
                web::get().to(get_macro_china_imports_yoy),
            )
            .route(
                "/china_trade_balance",
                web::get().to(get_macro_china_trade_balance),
            )
            .route(
                "/china_industrial_production_yoy",
                web::get().to(get_macro_china_industrial_production_yoy),
            )
            .route(
                "/china_cx_pmi_yearly",
                web::get().to(get_macro_china_cx_pmi_yearly),
            )
            .route(
                "/china_cx_services_pmi_yearly",
                web::get().to(get_macro_china_cx_services_pmi_yearly),
            )
            .route(
                "/china_non_man_pmi",
                web::get().to(get_macro_china_non_man_pmi),
            )
            .route(
                "/china_fx_reserves_yearly",
                web::get().to(get_macro_china_fx_reserves_yearly),
            )
            .route(
                "/usa_adp_employment",
                web::get().to(get_macro_usa_adp_employment),
            )
            .route(
                "/usa_initial_jobless",
                web::get().to(get_macro_usa_initial_jobless),
            )
            .route("/usa_ppi", web::get().to(get_macro_usa_ppi))
            .route("/usa_ism_pmi", web::get().to(get_macro_usa_ism_pmi))
            .route(
                "/usa_retail_sales",
                web::get().to(get_macro_usa_retail_sales),
            )
            .route(
                "/usa_industrial_production",
                web::get().to(get_macro_usa_industrial_production),
            )
            .route("/euro_gdp_yoy", web::get().to(get_macro_euro_gdp_yoy))
            .route("/euro_cpi_yoy", web::get().to(get_macro_euro_cpi_yoy))
            .route("/germany_ifo", web::get().to(get_macro_germany_ifo))
            .route(
                "/uk_gdp_quarterly",
                web::get().to(get_macro_uk_gdp_quarterly),
            )
            .route(
                "/australia_unemployment_rate",
                web::get().to(get_macro_australia_unemployment_rate),
            )
            .route("/cons_gold", web::get().to(get_macro_cons_gold))
            .route("/cons_silver", web::get().to(get_macro_cons_silver))
            .route("/cons_opec_month", web::get().to(get_macro_cons_opec_month))
            .route("/usa_cpi_monthly", web::get().to(get_macro_usa_cpi_monthly))
            .route(
                "/usa_core_cpi_monthly",
                web::get().to(get_macro_usa_core_cpi_monthly),
            )
            .route(
                "/usa_core_pce_price",
                web::get().to(get_macro_usa_core_pce_price),
            )
            .route(
                "/usa_trade_balance",
                web::get().to(get_macro_usa_trade_balance),
            )
            .route(
                "/usa_api_crude_stock",
                web::get().to(get_macro_usa_api_crude_stock),
            )
            .route("/usa_pmi", web::get().to(get_macro_usa_pmi))
            .route("/usa_ism_non_pmi", web::get().to(get_macro_usa_ism_non_pmi))
            .route(
                "/usa_house_starts",
                web::get().to(get_macro_usa_house_starts),
            )
            .route(
                "/usa_new_home_sales",
                web::get().to(get_macro_usa_new_home_sales),
            )
            .route(
                "/usa_building_permits",
                web::get().to(get_macro_usa_building_permits),
            )
            .route(
                "/usa_cb_consumer_confidence",
                web::get().to(get_macro_usa_cb_consumer_confidence),
            )
            .route(
                "/usa_michigan_consumer_sentiment",
                web::get().to(get_macro_usa_michigan_consumer_sentiment),
            )
            .route(
                "/euro_industrial_production_mom",
                web::get().to(get_macro_euro_industrial_production_mom),
            )
            .route(
                "/euro_manufacturing_pmi",
                web::get().to(get_macro_euro_manufacturing_pmi),
            )
            .route(
                "/euro_services_pmi",
                web::get().to(get_macro_euro_services_pmi),
            )
            .route(
                "/euro_zew_economic_sentiment",
                web::get().to(get_macro_euro_zew_economic_sentiment),
            )
            .route(
                "/germany_cpi_monthly",
                web::get().to(get_macro_germany_cpi_monthly),
            )
            .route("/germany_gdp", web::get().to(get_macro_germany_gdp))
            .route("/uk_cpi_monthly", web::get().to(get_macro_uk_cpi_monthly))
            .route(
                "/uk_unemployment_rate",
                web::get().to(get_macro_uk_unemployment_rate),
            )
            .route(
                "/australia_retail_rate_monthly",
                web::get().to(get_macro_australia_retail_rate_monthly),
            )
            .route(
                "/australia_ppi_quarterly",
                web::get().to(get_macro_australia_ppi_quarterly),
            )
            .route(
                "/canada_unemployment_rate",
                web::get().to(get_macro_canada_unemployment_rate),
            )
            .route(
                "/japan_cpi_yearly",
                web::get().to(get_macro_japan_cpi_yearly),
            )
            .route(
                "/swiss_cpi_yearly",
                web::get().to(get_macro_swiss_cpi_yearly),
            )
            .route("/swiss_svme", web::get().to(get_macro_swiss_svme))
            .route(
                "/china_urban_unemployment",
                web::get().to(get_macro_china_urban_unemployment),
            )
            .route(
                "/china_consumer_goods_retail",
                web::get().to(get_macro_china_consumer_goods_retail),
            )
            .route(
                "/china_cpi_monthly",
                web::get().to(get_macro_china_cpi_monthly),
            )
            .route(
                "/china_ppi_yearly",
                web::get().to(get_macro_china_ppi_yearly),
            )
            .route(
                "/china_pmi_yearly",
                web::get().to(get_macro_china_pmi_yearly),
            )
            .route("/china_m2_yearly", web::get().to(get_macro_china_m2_yearly))
            .route(
                "/usa_pending_home_sales",
                web::get().to(get_macro_usa_pending_home_sales),
            )
            .route(
                "/usa_exist_home_sales",
                web::get().to(get_macro_usa_exist_home_sales),
            )
            .route(
                "/usa_business_inventories",
                web::get().to(get_macro_usa_business_inventories),
            )
            .route(
                "/usa_factory_orders",
                web::get().to(get_macro_usa_factory_orders),
            )
            .route(
                "/canada_cpi_yearly",
                web::get().to(get_macro_canada_cpi_yearly),
            )
            .route(
                "/australia_cpi_quarterly",
                web::get().to(get_macro_australia_cpi_quarterly),
            )
            .route("/uk_trade", web::get().to(get_macro_uk_trade))
            .route(
                "/japan_core_cpi_yearly",
                web::get().to(get_macro_japan_core_cpi_yearly),
            )
            .route("/stock_finance", web::get().to(get_macro_stock_finance))
            .route("/china_hk_cpi", web::get().to(get_macro_china_hk_cpi))
            .route(
                "/china_hk_rate_of_unemployment",
                web::get().to(get_macro_china_hk_rate_of_unemployment),
            )
            .route("/china_hk_gbp", web::get().to(get_macro_china_hk_gbp))
            .route(
                "/china_hk_trade_diff_ratio",
                web::get().to(get_macro_china_hk_trade_diff_ratio),
            )
            .route(
                "/euro_deposit_rate",
                web::get().to(get_macro_euro_deposit_rate),
            )
            .route(
                "/euro_marginal_lending_rate",
                web::get().to(get_macro_euro_marginal_lending_rate),
            )
            .route(
                "/bank_canada_interest_rate",
                web::get().to(get_macro_bank_canada_interest_rate),
            )
            .route(
                "/usa_durable_goods_orders",
                web::get().to(get_macro_usa_durable_goods_orders),
            )
            .route(
                "/usa_personal_spending",
                web::get().to(get_macro_usa_personal_spending),
            )
            .route("/china_gdzctz", web::get().to(get_macro_china_gdzctz))
            .route("/china_hgjck", web::get().to(get_macro_china_hgjck))
            .route("/china_czsr", web::get().to(get_macro_china_czsr))
            .route("/china_whxd", web::get().to(get_macro_china_whxd))
            .route("/china_xfzxx", web::get().to(get_macro_china_xfzxx))
            .route(
                "/china_reserve_requirement_ratio",
                web::get().to(get_macro_china_reserve_requirement_ratio),
            )
            .route(
                "/canada_retail_rate_monthly",
                web::get().to(get_macro_canada_retail_rate_monthly),
            )
            .route(
                "/germany_retail_sale_monthly",
                web::get().to(get_macro_germany_retail_sale_monthly),
            )
            .route(
                "/uk_retail_monthly",
                web::get().to(get_macro_uk_retail_monthly),
            )
            .route("/swiss_trade", web::get().to(get_macro_swiss_trade))
            .route(
                "/germany_trade_adjusted",
                web::get().to(get_macro_germany_trade_adjusted),
            )
            .route("/canada_trade", web::get().to(get_macro_canada_trade))
            .route(
                "/swiss_gbd_yearly",
                web::get().to(get_macro_swiss_gbd_yearly),
            )
            .route(
                "/uk_halifax_monthly",
                web::get().to(get_macro_uk_halifax_monthly),
            )
            .route(
                "/uk_halifax_yearly",
                web::get().to(get_macro_uk_halifax_yearly),
            )
            .route(
                "/uk_rightmove_monthly",
                web::get().to(get_macro_uk_rightmove_monthly),
            )
            .route(
                "/uk_rightmove_yearly",
                web::get().to(get_macro_uk_rightmove_yearly),
            )
            .route(
                "/germany_retail_sale_yearly",
                web::get().to(get_macro_germany_retail_sale_yearly),
            )
            .route(
                "/canada_new_house_rate",
                web::get().to(get_macro_canada_new_house_rate),
            )
            .route(
                "/canada_bank_rate",
                web::get().to(get_macro_canada_bank_rate),
            )
            .route(
                "/china_hk_building_volume",
                web::get().to(get_macro_china_hk_building_volume),
            )
            .route(
                "/china_hk_building_amount",
                web::get().to(get_macro_china_hk_building_amount),
            )
            .route(
                "/canada_core_cpi_yearly",
                web::get().to(get_macro_canada_core_cpi_yearly),
            )
            .route(
                "/canada_core_cpi_monthly",
                web::get().to(get_macro_canada_core_cpi_monthly),
            )
            .route(
                "/canada_cpi_monthly",
                web::get().to(get_macro_canada_cpi_monthly),
            )
            .route(
                "/uk_core_cpi_yearly",
                web::get().to(get_macro_uk_core_cpi_yearly),
            )
            .route(
                "/uk_core_cpi_monthly",
                web::get().to(get_macro_uk_core_cpi_monthly),
            )
            .route("/uk_cpi_yearly", web::get().to(get_macro_uk_cpi_yearly))
            .route(
                "/germany_gdp_yearly",
                web::get().to(get_macro_germany_gdp_yearly),
            )
            .route("/usa_rig_count", web::get().to(get_macro_usa_rig_count))
            .route(
                "/usa_eia_crude_rate",
                web::get().to(get_macro_usa_eia_crude_rate),
            )
            .route("/rmb_loan", web::get().to(get_macro_rmb_loan))
            .route("/rmb_deposit", web::get().to(get_macro_rmb_deposit))
            .route("/info_ws", web::get().to(get_macro_info_ws)),
    );
}
