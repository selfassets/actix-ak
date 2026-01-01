use actix_web::{web, HttpResponse, Result};
use crate::models::{ApiResponse, StockInfo, StockHistoryData, StockQuery};
use crate::services::stock_service;

pub async fn get_stock_info(path: web::Path<String>) -> Result<HttpResponse> {
    let symbol = path.into_inner();
    
    match stock_service::get_stock_info(&symbol).await {
        Ok(stock_info) => {
            let response = ApiResponse::success(stock_info);
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            let response = ApiResponse::<StockInfo>::error(e.to_string());
            Ok(HttpResponse::InternalServerError().json(response))
        }
    }
}

pub async fn get_stock_history(
    path: web::Path<String>,
    query: web::Query<StockQuery>,
) -> Result<HttpResponse> {
    let symbol = path.into_inner();
    
    match stock_service::get_stock_history(&symbol, &query).await {
        Ok(history_data) => {
            let response = ApiResponse::success(history_data);
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            let response = ApiResponse::<Vec<StockHistoryData>>::error(e.to_string());
            Ok(HttpResponse::InternalServerError().json(response))
        }
    }
}

pub async fn list_stocks(query: web::Query<StockQuery>) -> Result<HttpResponse> {
    match stock_service::list_stocks(&query).await {
        Ok(stocks) => {
            let response = ApiResponse::success(stocks);
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            let response = ApiResponse::<Vec<StockInfo>>::error(e.to_string());
            Ok(HttpResponse::InternalServerError().json(response))
        }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/stocks")
            .route("", web::get().to(list_stocks))
            .route("/{symbol}", web::get().to(get_stock_info))
            .route("/{symbol}/history", web::get().to(get_stock_history))
    );
}