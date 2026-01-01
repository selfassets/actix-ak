use actix_web::{web, HttpResponse, Result};
use crate::models::{
    ApiResponse, FuturesInfo, FuturesHistoryData, FuturesQuery,
    FuturesSymbolMark, FuturesContractDetail, ForeignFuturesSymbol
};
use crate::services::futures_service::{
    FuturesService, get_futures_history, get_futures_minute_data,
    get_foreign_futures_symbols, get_foreign_futures_realtime
};

/// 获取单个期货合约实时数据
/// GET /futures/{symbol}
pub async fn get_futures_info(path: web::Path<String>) -> Result<HttpResponse> {
    let symbol = path.into_inner();
    let service = FuturesService::new();
    
    match service.get_futures_info(&symbol).await {
        Ok(futures_info) => {
            let response = ApiResponse::success(futures_info);
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            let response = ApiResponse::<FuturesInfo>::error(e.to_string());
            Ok(HttpResponse::InternalServerError().json(response))
        }
    }
}

/// 获取期货日K线历史数据
/// GET /futures/{symbol}/history?limit=30
pub async fn get_history(
    path: web::Path<String>,
    query: web::Query<FuturesQuery>,
) -> Result<HttpResponse> {
    let symbol = path.into_inner();
    
    match get_futures_history(&symbol, &query).await {
        Ok(history_data) => {
            let response = ApiResponse::success(history_data);
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            let response = ApiResponse::<Vec<FuturesHistoryData>>::error(e.to_string());
            Ok(HttpResponse::InternalServerError().json(response))
        }
    }
}

/// 获取期货分钟K线数据
/// GET /futures/{symbol}/minute?period=5
#[derive(serde::Deserialize)]
pub struct MinuteQuery {
    pub period: Option<String>,  // 1, 5, 15, 30, 60
}

pub async fn get_minute(
    path: web::Path<String>,
    query: web::Query<MinuteQuery>,
) -> Result<HttpResponse> {
    let symbol = path.into_inner();
    let period = query.period.as_deref().unwrap_or("5");
    
    match get_futures_minute_data(&symbol, period).await {
        Ok(minute_data) => {
            let response = ApiResponse::success(minute_data);
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            let response = ApiResponse::<Vec<FuturesHistoryData>>::error(e.to_string());
            Ok(HttpResponse::InternalServerError().json(response))
        }
    }
}

/// 获取期货列表（按交易所或品种）
/// GET /futures?exchange=SHFE&limit=20
pub async fn list_futures(query: web::Query<FuturesQuery>) -> Result<HttpResponse> {
    let mut service = FuturesService::new();
    
    match service.list_main_futures(&query).await {
        Ok(futures_list) => {
            let response = ApiResponse::success(futures_list);
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            let response = ApiResponse::<Vec<FuturesInfo>>::error(e.to_string());
            Ok(HttpResponse::InternalServerError().json(response))
        }
    }
}

/// 获取支持的交易所列表
/// GET /futures/exchanges
pub async fn get_exchanges() -> Result<HttpResponse> {
    let service = FuturesService::new();
    let exchanges = service.get_exchanges();
    let response = ApiResponse::success(exchanges);
    Ok(HttpResponse::Ok().json(response))
}

/// 批量获取期货实时数据
/// POST /futures/batch
pub async fn get_multiple_futures(
    _query: web::Query<FuturesQuery>,
    body: web::Json<Vec<String>>,
) -> Result<HttpResponse> {
    let symbols = body.into_inner();
    let service = FuturesService::new();
    
    if symbols.is_empty() {
        let response = ApiResponse::<Vec<FuturesInfo>>::error("合约代码列表不能为空".to_string());
        return Ok(HttpResponse::BadRequest().json(response));
    }
    
    match service.get_multiple_futures(&symbols).await {
        Ok(futures_list) => {
            let response = ApiResponse::success(futures_list);
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            let response = ApiResponse::<Vec<FuturesInfo>>::error(e.to_string());
            Ok(HttpResponse::InternalServerError().json(response))
        }
    }
}

/// 获取期货品种映射表
/// GET /futures/symbols
pub async fn get_symbol_mark() -> Result<HttpResponse> {
    let mut service = FuturesService::new();
    
    match service.get_symbol_mark().await {
        Ok(symbols) => {
            let response = ApiResponse::success(symbols);
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            let response = ApiResponse::<Vec<FuturesSymbolMark>>::error(e.to_string());
            Ok(HttpResponse::InternalServerError().json(response))
        }
    }
}

/// 获取指定交易所的品种列表
/// GET /futures/symbols/{exchange}
pub async fn get_exchange_symbols(path: web::Path<String>) -> Result<HttpResponse> {
    let exchange = path.into_inner();
    let mut service = FuturesService::new();
    
    match service.get_exchange_symbols(&exchange).await {
        Ok(symbols) => {
            let response = ApiResponse::success(symbols);
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            let response = ApiResponse::<Vec<FuturesSymbolMark>>::error(e.to_string());
            Ok(HttpResponse::InternalServerError().json(response))
        }
    }
}

/// 获取交易所主力合约列表
/// GET /futures/main/{exchange}
pub async fn get_main_contracts(path: web::Path<String>) -> Result<HttpResponse> {
    let exchange = path.into_inner();
    let mut service = FuturesService::new();
    
    match service.get_main_contracts(&exchange).await {
        Ok(contracts) => {
            let response = ApiResponse::success(contracts);
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            let response = ApiResponse::<Vec<String>>::error(e.to_string());
            Ok(HttpResponse::InternalServerError().json(response))
        }
    }
}

/// 获取合约详情
/// GET /futures/{symbol}/detail
pub async fn get_contract_detail(path: web::Path<String>) -> Result<HttpResponse> {
    let symbol = path.into_inner();
    let service = FuturesService::new();
    
    match service.get_contract_detail(&symbol).await {
        Ok(detail) => {
            let response = ApiResponse::success(detail);
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            let response = ApiResponse::<FuturesContractDetail>::error(e.to_string());
            Ok(HttpResponse::InternalServerError().json(response))
        }
    }
}

/// 获取外盘期货品种列表
/// GET /futures/foreign/symbols
pub async fn get_foreign_symbols() -> Result<HttpResponse> {
    let symbols = get_foreign_futures_symbols();
    let response = ApiResponse::success(symbols);
    Ok(HttpResponse::Ok().json(response))
}

/// 获取外盘期货实时行情
/// POST /futures/foreign/realtime
pub async fn get_foreign_realtime(body: web::Json<Vec<String>>) -> Result<HttpResponse> {
    let codes = body.into_inner();
    
    if codes.is_empty() {
        let response = ApiResponse::<Vec<FuturesInfo>>::error("品种代码列表不能为空".to_string());
        return Ok(HttpResponse::BadRequest().json(response));
    }
    
    match get_foreign_futures_realtime(&codes).await {
        Ok(futures_list) => {
            let response = ApiResponse::success(futures_list);
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            let response = ApiResponse::<Vec<FuturesInfo>>::error(e.to_string());
            Ok(HttpResponse::InternalServerError().json(response))
        }
    }
}

/// 获取品种所有合约实时数据
/// GET /futures/realtime/{symbol}
pub async fn get_realtime_by_symbol(path: web::Path<String>) -> Result<HttpResponse> {
    let symbol = path.into_inner();
    let mut service = FuturesService::new();
    
    match service.get_futures_realtime_by_symbol(&symbol).await {
        Ok(futures_list) => {
            let response = ApiResponse::success(futures_list);
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            let response = ApiResponse::<Vec<FuturesInfo>>::error(e.to_string());
            Ok(HttpResponse::InternalServerError().json(response))
        }
    }
}

/// 配置期货相关路由
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/futures")
            // 列表和基础信息
            .route("", web::get().to(list_futures))
            .route("/exchanges", web::get().to(get_exchanges))
            .route("/symbols", web::get().to(get_symbol_mark))
            .route("/symbols/{exchange}", web::get().to(get_exchange_symbols))
            .route("/main/{exchange}", web::get().to(get_main_contracts))
            .route("/batch", web::post().to(get_multiple_futures))
            // 外盘期货
            .route("/foreign/symbols", web::get().to(get_foreign_symbols))
            .route("/foreign/realtime", web::post().to(get_foreign_realtime))
            // 品种实时数据
            .route("/realtime/{symbol}", web::get().to(get_realtime_by_symbol))
            // 单个合约
            .route("/{symbol}", web::get().to(get_futures_info))
            .route("/{symbol}/history", web::get().to(get_history))
            .route("/{symbol}/minute", web::get().to(get_minute))
            .route("/{symbol}/detail", web::get().to(get_contract_detail))
    );
}
