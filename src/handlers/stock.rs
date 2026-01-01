//! 股票接口处理器
//! 
//! 提供股票数据的 HTTP API 端点

use actix_web::{web, HttpResponse, Result};
use crate::models::{ApiResponse, StockInfo, StockHistoryData, StockQuery};
use crate::services::stock_service;

/// 获取单只股票信息
/// 
/// GET /api/v1/stocks/{symbol}
/// 
/// # 参数
/// - symbol: 股票代码
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

/// 获取股票历史K线数据
/// 
/// GET /api/v1/stocks/{symbol}/history?limit=30
/// 
/// # 参数
/// - symbol: 股票代码
/// - limit: 返回数量限制（可选，默认30）
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

/// 获取股票列表
/// 
/// GET /api/v1/stocks?limit=20
/// 
/// # 参数
/// - limit: 返回数量限制（可选）
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

/// 配置股票相关路由
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/stocks")
            .route("", web::get().to(list_stocks))                    // 股票列表
            .route("/{symbol}", web::get().to(get_stock_info))        // 单只股票信息
            .route("/{symbol}/history", web::get().to(get_stock_history))  // 历史K线
    );
}