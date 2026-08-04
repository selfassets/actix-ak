//! 各大期货交易所日终结算价 HTTP 处理器

use crate::models::ak::macro_data::MacroItem;
use crate::models::ApiResponse;
use crate::services::futures::settle;
use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct FuturesSettleQuery {
    pub date: Option<String>,
}

/// 获取中金所(CFFEX)日终结算价数据
///
/// GET /api/v1/ak/futures/settle_cffex
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/futures/settle_cffex",
        tag = "futures",
        params(FuturesSettleQuery),
        responses(
            (status = 200, description = "成功获取中金所结算数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_futures_settle_cffex(
    query: web::Query<FuturesSettleQuery>,
) -> Result<HttpResponse> {
    let date = query.date.clone().unwrap_or_else(|| "20260119".to_string());
    match settle::futures_settle_cffex(&date).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取郑商所(CZCE)日终结算价数据
///
/// GET /api/v1/ak/futures/settle_czce
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/futures/settle_czce",
        tag = "futures",
        params(FuturesSettleQuery),
        responses(
            (status = 200, description = "成功获取郑商所结算数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_futures_settle_czce(
    query: web::Query<FuturesSettleQuery>,
) -> Result<HttpResponse> {
    let date = query.date.clone().unwrap_or_else(|| "20260119".to_string());
    match settle::futures_settle_czce(&date).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取大商所(DCE)日终结算价数据
///
/// GET /api/v1/ak/futures/settle_dce
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/futures/settle_dce",
        tag = "futures",
        params(FuturesSettleQuery),
        responses(
            (status = 200, description = "成功获取大商所结算数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_futures_settle_dce(query: web::Query<FuturesSettleQuery>) -> Result<HttpResponse> {
    let date = query.date.clone().unwrap_or_else(|| "20260119".to_string());
    match settle::futures_settle_dce(&date).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}
/// 获取 COMEX 黄金/白银堆存数据
///
/// GET /api/v1/ak/futures/comex_inventory
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/futures/comex_inventory",
        tag = "futures",
        params(
            ("symbol" = Option<String>, Query, description = "品种，如 黄金、白银")
        ),
        responses(
            (status = 200, description = "成功获取 COMEX 堆存数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_futures_comex_inventory(
    query: web::Query<crate::models::ak::fortune::FortuneRankQuery>,
) -> Result<HttpResponse> {
    let symbol = query.symbol.clone().unwrap_or_else(|| "黄金".to_string());
    match crate::services::futures::futures_comex_inventory(&symbol).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取东方财富国内期货库存数据
///
/// GET /api/v1/ak/futures/inventory_em
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/futures/inventory_em",
        tag = "futures",
        params(
            ("symbol" = Option<String>, Query, description = "品种代码，如 a")
        ),
        responses(
            (status = 200, description = "成功获取东财期货库存数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_futures_inventory_em(
    query: web::Query<crate::models::ak::fortune::FortuneRankQuery>,
) -> Result<HttpResponse> {
    let symbol = query.symbol.clone().unwrap_or_else(|| "a".to_string());
    match crate::services::futures::futures_inventory_em(&symbol).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取上海金属网(SHMET)期货快讯资讯
///
/// GET /api/v1/ak/futures/news_shmet
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/futures/news_shmet",
        tag = "futures",
        params(
            ("symbol" = Option<String>, Query, description = "关键词，如 铜、铝、全部")
        ),
        responses(
            (status = 200, description = "成功获取上海金属网资讯", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_futures_news_shmet(
    query: web::Query<crate::models::ak::fortune::FortuneRankQuery>,
) -> Result<HttpResponse> {
    let symbol = query.symbol.clone().unwrap_or_else(|| "全部".to_string());
    match crate::services::futures::futures_news_shmet(&symbol).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取中证商品期货指数数据
///
/// GET /api/v1/ak/futures/index_ccidx
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/futures/index_ccidx",
        tag = "futures",
        params(
            ("symbol" = Option<String>, Query, description = "指数名称，如 中证商品期货指数")
        ),
        responses(
            (status = 200, description = "成功获取中证商品期货指数", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_futures_index_ccidx(
    query: web::Query<crate::models::ak::fortune::FortuneRankQuery>,
) -> Result<HttpResponse> {
    let symbol = query
        .symbol
        .clone()
        .unwrap_or_else(|| "中证商品期货指数".to_string());
    match crate::services::futures::futures_index_ccidx(&symbol).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取东方财富期货交易规则与保证金参数表
///
/// GET /api/v1/ak/futures/rule_em
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/futures/rule_em",
        tag = "futures",
        responses(
            (status = 200, description = "成功获取期货交易规则表", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_futures_rule_em() -> Result<HttpResponse> {
    match crate::services::futures::futures_rule_em().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取新加坡交易所(SGX)日终结算价数据
///
/// GET /api/v1/ak/futures/settlement_price_sgx
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/futures/settlement_price_sgx",
        tag = "futures",
        params(FuturesSettleQuery),
        responses(
            (status = 200, description = "成功获取新加坡交易所结算数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_futures_settlement_price_sgx(
    query: web::Query<FuturesSettleQuery>,
) -> Result<HttpResponse> {
    let date = query.date.clone().unwrap_or_else(|| "20231107".to_string());
    match crate::services::futures::futures_settlement_price_sgx(&date).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取东方财富期货与现货股票对照表数据
///
/// GET /api/v1/ak/futures/spot_stock
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/futures/spot_stock",
        tag = "futures",
        params(
            ("symbol" = Option<String>, Query, description = "板块名称，如 能源、金属")
        ),
        responses(
            (status = 200, description = "成功获取期货现货股票对照表", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_futures_spot_stock(
    query: web::Query<crate::models::ak::fortune::FortuneRankQuery>,
) -> Result<HttpResponse> {
    let symbol = query.symbol.clone().unwrap_or_else(|| "能源".to_string());
    match crate::services::futures::futures_spot_stock(&symbol).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取生意社指定日期大宗商品现货价格及基差数据
///
/// GET /api/v1/ak/futures/spot_price
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/futures/spot_price",
        tag = "futures",
        params(FuturesSettleQuery),
        responses(
            (status = 200, description = "成功获取现货基差数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_futures_spot_price(query: web::Query<FuturesSettleQuery>) -> Result<HttpResponse> {
    let date = query.date.clone().unwrap_or_else(|| "20240430".to_string());
    match crate::services::futures::get_futures_spot_price(&date, None).await {
        Ok(data) => {
            let items: Vec<MacroItem> = data
                .into_iter()
                .map(|item| {
                    let mut map = std::collections::HashMap::new();
                    map.insert(
                        "var".to_string(),
                        serde_json::Value::String(item.symbol.clone()),
                    );
                    map.insert("sp".to_string(), serde_json::json!(item.spot_price));
                    map.insert(
                        "near_symbol".to_string(),
                        serde_json::Value::String(item.near_contract),
                    );
                    map.insert(
                        "near_price".to_string(),
                        serde_json::json!(item.near_contract_price),
                    );
                    map.insert(
                        "dom_symbol".to_string(),
                        serde_json::Value::String(item.dominant_contract),
                    );
                    map.insert(
                        "dom_price".to_string(),
                        serde_json::json!(item.dominant_contract_price),
                    );
                    map.insert("near_basis".to_string(), serde_json::json!(item.near_basis));
                    map.insert("dom_basis".to_string(), serde_json::json!(item.dom_basis));
                    map.insert(
                        "near_basis_rate".to_string(),
                        serde_json::json!(item.near_basis_rate),
                    );
                    map.insert(
                        "dom_basis_rate".to_string(),
                        serde_json::json!(item.dom_basis_rate),
                    );
                    map.insert("date".to_string(), serde_json::Value::String(item.date));
                    MacroItem { data: map }
                })
                .collect();
            Ok(HttpResponse::Ok().json(ApiResponse::success(items)))
        }
        Err(err) => {
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err.to_string())))
        }
    }
}
/// 获取郑商所(CZCE)期转现明细数据
///
/// GET /api/v1/ak/futures/to_spot_czce
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/futures/to_spot_czce",
        tag = "futures",
        params(FuturesSettleQuery),
        responses(
            (status = 200, description = "成功获取郑商所期转现数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_futures_to_spot_czce(
    query: web::Query<FuturesSettleQuery>,
) -> Result<HttpResponse> {
    let date = query.date.clone().unwrap_or_else(|| "20231228".to_string());
    match settle::futures_to_spot_czce(&date).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}
/// 获取广期所(GFEX)日终结算价数据
///
/// GET /api/v1/ak/futures/settle_gfex
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/futures/settle_gfex",
        tag = "futures",
        params(FuturesSettleQuery),
        responses(
            (status = 200, description = "成功获取广期所结算数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_futures_settle_gfex(
    query: web::Query<FuturesSettleQuery>,
) -> Result<HttpResponse> {
    let date = query.date.clone().unwrap_or_else(|| "20260119".to_string());
    match settle::futures_settle_gfex(&date).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取上海国际能源交易中心(INE)日终结算价数据
///
/// GET /api/v1/ak/futures/settle_ine
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/futures/settle_ine",
        tag = "futures",
        params(FuturesSettleQuery),
        responses(
            (status = 200, description = "成功获取上期能源结算数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_futures_settle_ine(query: web::Query<FuturesSettleQuery>) -> Result<HttpResponse> {
    let date = query.date.clone().unwrap_or_else(|| "20260119".to_string());
    match settle::futures_settle_ine(&date).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取东方财富全球外盘期货实时行情全量
///
/// GET /api/v1/ak/futures/global_spot_em
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/futures/global_spot_em",
        tag = "futures",
        responses(
            (status = 200, description = "成功获取东财外盘期货实时行情", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_futures_global_spot_em() -> Result<HttpResponse> {
    match crate::services::futures::futures_global_spot_em().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取上期所(SHFE)期转现明细数据
///
/// GET /api/v1/ak/futures/to_spot_shfe
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/futures/to_spot_shfe",
        tag = "futures",
        params(FuturesSettleQuery),
        responses(
            (status = 200, description = "成功获取上期所期转现数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_futures_to_spot_shfe(
    query: web::Query<FuturesSettleQuery>,
) -> Result<HttpResponse> {
    let date = query.date.clone().unwrap_or_else(|| "202312".to_string());
    match settle::futures_to_spot_shfe(&date).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取大商所(DCE)期转现明细数据
///
/// GET /api/v1/ak/futures/to_spot_dce
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/futures/to_spot_dce",
        tag = "futures",
        params(FuturesSettleQuery),
        responses(
            (status = 200, description = "成功获取大商所期转现数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_futures_to_spot_dce(
    query: web::Query<FuturesSettleQuery>,
) -> Result<HttpResponse> {
    let date = query.date.clone().unwrap_or_else(|| "202312".to_string());
    match settle::futures_to_spot_dce(&date).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}
/// 获取上期所(SHFE)日终结算价数据
///
/// GET /api/v1/ak/futures/settle_shfe
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/futures/settle_shfe",
        tag = "futures",
        params(FuturesSettleQuery),
        responses(
            (status = 200, description = "成功获取上期所结算数据", body = ApiResponse<Vec<MacroItem>>)
        )
    )
)]
pub async fn get_futures_settle_shfe(
    query: web::Query<FuturesSettleQuery>,
) -> Result<HttpResponse> {
    let date = query.date.clone().unwrap_or_else(|| "20260119".to_string());
    match settle::futures_settle_shfe(&date).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/futures")
            .route("/settle_cffex", web::get().to(get_futures_settle_cffex))
            .route("/settle_czce", web::get().to(get_futures_settle_czce))
            .route("/settle_dce", web::get().to(get_futures_settle_dce))
            .route("/settle_shfe", web::get().to(get_futures_settle_shfe))
            .route("/settle_gfex", web::get().to(get_futures_settle_gfex))
            .route("/settle_ine", web::get().to(get_futures_settle_ine))
            .route("/global_spot_em", web::get().to(get_futures_global_spot_em))
            .route("/to_spot_shfe", web::get().to(get_futures_to_spot_shfe))
            .route("/to_spot_dce", web::get().to(get_futures_to_spot_dce))
            .route("/to_spot_czce", web::get().to(get_futures_to_spot_czce))
            .route(
                "/comex_inventory",
                web::get().to(get_futures_comex_inventory),
            )
            .route("/inventory_em", web::get().to(get_futures_inventory_em))
            .route("/news_shmet", web::get().to(get_futures_news_shmet))
            .route("/index_ccidx", web::get().to(get_futures_index_ccidx))
            .route("/rule_em", web::get().to(get_futures_rule_em))
            .route(
                "/settlement_price_sgx",
                web::get().to(get_futures_settlement_price_sgx),
            )
            .route("/spot_stock", web::get().to(get_futures_spot_stock))
            .route("/spot_price", web::get().to(get_futures_spot_price)),
    );
}
