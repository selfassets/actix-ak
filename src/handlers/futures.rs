use actix_web::{web, HttpResponse, Result};
use crate::models::{
    ApiResponse, FuturesInfo, FuturesHiuturesService;

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

pub async fn get_futures_history(
    path: web::Path<String>,
    query: web::Query<FuturesQuery>,
) -> Result<HttpResponse> {
    let symbol = path.into_inner();
    
    match crate::services::futures_service::get_futures_history(&symbol, &query).await {
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

pub async fn get_exchanges() -> Result<HttpResponse> {
    let service = FuturesService::new();
    let exchanges = service.get_exchanges();
    let response = ApiResponse::success(exchanges);
    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_multiple_futures(
    _query: web::Query<FuturesQuery>,
    body: web::Json<Vec<String>>,
) -> Result<HttpResponse> {
    let symbols = body.into_inner();
    let service = FuturesService::new();
    
    if symbols.is_empty() {
        let response = ApiResponse::<Vec<FuturesInfo>>::error("Symbols list cannot be empty".to_string());
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

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/futures")
            .route("", web::get().to(list_futures))
            .route("/exchanges", web::get().to(get_exchanges))
            .route("/batch", web::post().to(get_multiple_futures))
            .route("/{symbol}", web::get().to(get_futures_info))
            .route("/{symbol}/history", web::get().to(get_futures_history))
    );
}