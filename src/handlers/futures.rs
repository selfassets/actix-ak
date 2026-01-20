//! 期货接口处理器
//! 
//! 提供期货数据的 HTTP API 端点
//! 
//! ## API 列表
//! 
//! ### 基础接口
//! - GET /futures - 获取期货列表
//! - GET /futures/{symbol} - 获取单个合约实时数据
//! - GET /futures/{symbol}/history - 获取日K线数据
//! - GET /futures/{symbol}/minute - 获取分钟K线数据
//! - GET /futures/{symbol}/detail - 获取合约详情
//! 
//! ### 品种和交易所
//! - GET /futures/exchanges - 获取交易所列表
//! - GET /futures/symbols - 获取品种映射表
//! - GET /futures/symbols/{exchange} - 获取指定交易所品种
//! 
//! ### 主力连续合约
//! - GET /futures/main/display - 获取主力连续合约一览
//! - GET /futures/main/{symbol}/daily - 获取主力连续日K线
//! 
//! ### 持仓和费用
//! - GET /futures/hold_pos - 获取持仓排名
//! - GET /futures/fees - 获取交易费用
//! - GET /futures/rule - 获取交易规则
//! 
//! ### 现货价格
//! - GET /futures/spot_price - 获取现货价格及基差
//! - GET /futures/spot_price_previous - 获取历史现货价格
//! - GET /futures/spot_price_daily - 获取现货价格日线

use actix_web::{web, HttpResponse, Result};
use crate::models::{
    ApiResponse, FuturesInfo, FuturesHistoryData, FuturesQuery,
    FuturesSymbolMark, FuturesContractDetail,
    FuturesMainContract, FuturesMainDailyData, FuturesHoldPosition,
    FuturesHoldPosQuery, FuturesMainQuery,
    ForeignFuturesHistData, ForeignFuturesDetail, FuturesFeesInfo,
    FuturesCommInfo, FuturesCommQuery, FuturesRule, FuturesRuleQuery,
    Futures99Symbol, FuturesInventory99, FuturesInventory99Query,
    FuturesSpotPrice, FuturesSpotPriceQuery,
    FuturesSpotPricePrevious, FuturesSpotPricePreviousQuery,
    FuturesSpotPriceDailyQuery, RankTableQuery, RankSumDailyQuery, RankTableResponse,
    RankSum, CzceWarehouseReceiptResponse, DceWarehouseReceipt,
    ShfeWarehouseReceiptResponse, GfexWarehouseReceiptResponse
};
use crate::services::futures::{
    FuturesService, get_futures_history, get_futures_minute_data,
    get_foreign_futures_symbols, get_foreign_futures_realtime,
    get_futures_display_main_sina, get_futures_main_sina, get_futures_hold_pos_sina,
    get_futures_foreign_hist, get_futures_foreign_detail, get_futures_fees_info,
    get_futures_comm_info, get_futures_rule,
    get_99_symbol_map, get_futures_inventory_99, get_futures_spot_price,
    get_futures_spot_price_previous, get_futures_spot_price_daily,
    get_shfe_rank_table, get_cffex_rank_table, get_dce_rank_table, get_rank_table_czce,
    get_gfex_rank_table, get_rank_sum, get_rank_sum_daily,
    futures_warehouse_receipt_czce, futures_warehouse_receipt_dce,
    futures_shfe_warehouse_receipt, futures_gfex_warehouse_receipt
};

/// 获取单个期货合约实时数据
/// 
/// GET /api/v1/futures/{symbol}
/// 
/// # 参数
/// - symbol: 合约代码（如 RB2510）
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

/// 获取上期所持仓排名表
/// GET /futures/rank/shfe?date=20240102&vars=CU,AL
pub async fn get_rank_shfe(query: web::Query<RankTableQuery>) -> Result<HttpResponse> {
    let vars = query
        .vars
        .as_ref()
        .map(|v| v.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect());

    match get_shfe_rank_table(&query.date, vars).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<Vec<RankTableResponse>>::error(e.to_string()))),
    }
}

/// 获取中金所持仓排名表
/// GET /futures/rank/cffex?date=20240102&vars=IF,IC
pub async fn get_rank_cffex(query: web::Query<RankTableQuery>) -> Result<HttpResponse> {
    let vars = query
        .vars
        .as_ref()
        .map(|v| v.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect());

    match get_cffex_rank_table(&query.date, vars).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<Vec<RankTableResponse>>::error(e.to_string()))),
    }
}

/// 获取大商所持仓排名表
/// GET /futures/rank/dce?date=20240102&vars=M,Y
pub async fn get_rank_dce(query: web::Query<RankTableQuery>) -> Result<HttpResponse> {
    let vars = query
        .vars
        .as_ref()
        .map(|v| v.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect());

    match get_dce_rank_table(&query.date, vars).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<Vec<RankTableResponse>>::error(e.to_string()))),
    }
}

/// 获取郑商所持仓排名表
/// GET /futures/rank/czce?date=20240102
pub async fn get_rank_czce(query: web::Query<RankTableQuery>) -> Result<HttpResponse> {
    match get_rank_table_czce(&query.date).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<Vec<RankTableResponse>>::error(e.to_string()))),
    }
}

/// 获取广期所持仓排名表
/// GET /futures/rank/gfex?date=20240102&vars=SI,LC
pub async fn get_rank_gfex(query: web::Query<RankTableQuery>) -> Result<HttpResponse> {
    let vars = query
        .vars
        .as_ref()
        .map(|v| v.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect());

    match get_gfex_rank_table(&query.date, vars).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<Vec<RankTableResponse>>::error(e.to_string()))),
    }
}

/// 获取持仓排名汇总
/// GET /futures/rank/sum?date=20240102&vars=CU,AL
pub async fn get_rank_sum_data(query: web::Query<RankTableQuery>) -> Result<HttpResponse> {
    let vars = query
        .vars
        .as_ref()
        .map(|v| v.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect());

    match get_rank_sum(&query.date, vars).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<Vec<RankSum>>::error(e.to_string()))),
    }
}

/// 获取持仓排名汇总（日期区间）
/// GET /futures/rank/sum_daily?start_date=20240102&end_date=20240110&vars=CU,AL
pub async fn get_rank_sum_daily_data(query: web::Query<RankSumDailyQuery>) -> Result<HttpResponse> {
    let vars = query
        .vars
        .as_ref()
        .map(|v| v.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect());

    match get_rank_sum_daily(&query.start_date, &query.end_date, vars).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<Vec<RankSum>>::error(e.to_string()))),
    }
}

/// 获取郑商所仓单日报
/// GET /futures/warehouse/czce?date=20240102
pub async fn get_warehouse_czce(query: web::Query<RankTableQuery>) -> Result<HttpResponse> {
    match futures_warehouse_receipt_czce(&query.date).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<Vec<CzceWarehouseReceiptResponse>>::error(e.to_string()))),
    }
}

/// 获取大商所仓单日报
/// GET /futures/warehouse/dce?date=20240102
pub async fn get_warehouse_dce(query: web::Query<RankTableQuery>) -> Result<HttpResponse> {
    match futures_warehouse_receipt_dce(&query.date).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<Vec<DceWarehouseReceipt>>::error(e.to_string()))),
    }
}

/// 获取上期所仓单日报
/// GET /futures/warehouse/shfe?date=20240102
pub async fn get_warehouse_shfe(query: web::Query<RankTableQuery>) -> Result<HttpResponse> {
    match futures_shfe_warehouse_receipt(&query.date).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<Vec<ShfeWarehouseReceiptResponse>>::error(e.to_string()))),
    }
}

/// 获取广期所仓单日报
/// GET /futures/warehouse/gfex?date=20240102
pub async fn get_warehouse_gfex(query: web::Query<RankTableQuery>) -> Result<HttpResponse> {
    match futures_gfex_warehouse_receipt(&query.date).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<Vec<GfexWarehouseReceiptResponse>>::error(e.to_string()))),
    }
}

/// 获取期货日K线历史数据
/// 
/// GET /api/v1/futures/{symbol}/history?limit=30
/// 
/// # 参数
/// - symbol: 合约代码
/// - limit: 返回数量限制（可选，默认30）
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

/// 获取主力连续合约一览表
/// GET /futures/main/display
/// 对应 akshare 的 futures_display_main_sina()
pub async fn get_display_main_contracts() -> Result<HttpResponse> {
    match get_futures_display_main_sina().await {
        Ok(contracts) => {
            let response = ApiResponse::success(contracts);
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            let response = ApiResponse::<Vec<FuturesMainContract>>::error(e.to_string());
            Ok(HttpResponse::InternalServerError().json(response))
        }
    }
}

/// 获取主力连续合约日K线数据
/// GET /futures/main/{symbol}/daily?start_date=20240101&end_date=20240301
/// 对应 akshare 的 futures_main_sina()
pub async fn get_main_daily(
    path: web::Path<String>,
    query: web::Query<FuturesMainQuery>,
) -> Result<HttpResponse> {
    let symbol = path.into_inner();
    
    match get_futures_main_sina(
        &symbol,
        query.start_date.as_deref(),
        query.end_date.as_deref(),
    ).await {
        Ok(data) => {
            let response = ApiResponse::success(data);
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            let response = ApiResponse::<Vec<FuturesMainDailyData>>::error(e.to_string());
            Ok(HttpResponse::InternalServerError().json(response))
        }
    }
}

/// 获取期货持仓排名数据
/// GET /futures/hold_pos?pos_type=volume&contract=RB2510&date=20250107
/// 对应 akshare 的 futures_hold_pos_sina()
pub async fn get_hold_pos(query: web::Query<FuturesHoldPosQuery>) -> Result<HttpResponse> {
    let pos_type = query.pos_type.as_deref().unwrap_or("volume");
    
    match get_futures_hold_pos_sina(pos_type, &query.contract, &query.date).await {
        Ok(positions) => {
            let response = ApiResponse::success(positions);
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            let response = ApiResponse::<Vec<FuturesHoldPosition>>::error(e.to_string());
            Ok(HttpResponse::InternalServerError().json(response))
        }
    }
}

/// 获取外盘期货历史数据（日K线）
/// GET /futures/foreign/{symbol}/history
/// 对应 akshare 的 futures_foreign_hist()
pub async fn get_foreign_history(path: web::Path<String>) -> Result<HttpResponse> {
    let symbol = path.into_inner();
    
    match get_futures_foreign_hist(&symbol).await {
        Ok(data) => {
            let response = ApiResponse::success(data);
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            let response = ApiResponse::<Vec<ForeignFuturesHistData>>::error(e.to_string());
            Ok(HttpResponse::InternalServerError().json(response))
        }
    }
}

/// 获取外盘期货合约详情
/// GET /futures/foreign/{symbol}/detail
/// 对应 akshare 的 futures_foreign_detail()
pub async fn get_foreign_detail(path: web::Path<String>) -> Result<HttpResponse> {
    let symbol = path.into_inner();
    
    match get_futures_foreign_detail(&symbol).await {
        Ok(detail) => {
            let response = ApiResponse::success(detail);
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            let response = ApiResponse::<ForeignFuturesDetail>::error(e.to_string());
            Ok(HttpResponse::InternalServerError().json(response))
        }
    }
}

/// 获取期货交易费用参照表
/// GET /futures/fees
/// 对应 akshare 的 futures_fees_info()
pub async fn get_fees_info() -> Result<HttpResponse> {
    match get_futures_fees_info().await {
        Ok(fees) => {
            let response = ApiResponse::success(fees);
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            let response = ApiResponse::<Vec<FuturesFeesInfo>>::error(e.to_string());
            Ok(HttpResponse::InternalServerError().json(response))
        }
    }
}

/// 获取期货手续费信息（九期网）
/// GET /futures/comm_info?exchange=所有
/// 对应 akshare 的 futures_comm_info()
pub async fn get_comm_info(query: web::Query<FuturesCommQuery>) -> Result<HttpResponse> {
    let exchange = query.exchange.as_deref();
    
    match get_futures_comm_info(exchange).await {
        Ok(data) => {
            let response = ApiResponse::success(data);
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            let response = ApiResponse::<Vec<FuturesCommInfo>>::error(e.to_string());
            Ok(HttpResponse::InternalServerError().json(response))
        }
    }
}

/// 获取期货交易规则
/// GET /futures/rule?date=20250328
/// 对应 akshare 的 futures_rule()
pub async fn get_rule(query: web::Query<FuturesRuleQuery>) -> Result<HttpResponse> {
    let date = query.date.as_deref();
    
    match get_futures_rule(date).await {
        Ok(rules) => {
            let response = ApiResponse::success(rules);
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            let response = ApiResponse::<Vec<FuturesRule>>::error(e.to_string());
            Ok(HttpResponse::InternalServerError().json(response))
        }
    }
}

/// 获取99期货网品种映射表
/// GET /futures/inventory99/symbols
pub async fn get_inventory99_symbols() -> Result<HttpResponse> {
    match get_99_symbol_map().await {
        Ok(symbols) => {
            let response = ApiResponse::success(symbols);
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            let response = ApiResponse::<Vec<Futures99Symbol>>::error(e.to_string());
            Ok(HttpResponse::InternalServerError().json(response))
        }
    }
}

/// 获取99期货网库存数据
/// GET /futures/inventory99?symbol=豆一
/// 对应 akshare 的 futures_inventory_99()
pub async fn get_inventory99(query: web::Query<FuturesInventory99Query>) -> Result<HttpResponse> {
    match get_futures_inventory_99(&query.symbol).await {
        Ok(data) => {
            let response = ApiResponse::success(data);
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            let response = ApiResponse::<Vec<FuturesInventory99>>::error(e.to_string());
            Ok(HttpResponse::InternalServerError().json(response))
        }
    }
}

/// 获取期货现货价格及基差数据
/// GET /futures/spot_price?date=20240430&symbols=RB,CU
/// 对应 akshare 的 futures_spot_price()
pub async fn get_spot_price(query: web::Query<FuturesSpotPriceQuery>) -> Result<HttpResponse> {
    let symbols: Option<Vec<&str>> = query.symbols.as_ref()
        .map(|s| s.split(',').map(|x| x.trim()).collect());
    
    match get_futures_spot_price(&query.date, symbols).await {
        Ok(data) => {
            let response = ApiResponse::success(data);
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            let response = ApiResponse::<Vec<FuturesSpotPrice>>::error(e.to_string());
            Ok(HttpResponse::InternalServerError().json(response))
        }
    }
}

/// 获取期货现货价格及基差历史数据（包含180日统计）
/// GET /futures/spot_price_previous?date=20240430
/// 对应 akshare 的 futures_spot_price_previous()
pub async fn get_spot_price_previous(query: web::Query<FuturesSpotPricePreviousQuery>) -> Result<HttpResponse> {
    match get_futures_spot_price_previous(&query.date).await {
        Ok(data) => {
            let response = ApiResponse::success(data);
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            let response = ApiResponse::<Vec<FuturesSpotPricePrevious>>::error(e.to_string());
            Ok(HttpResponse::InternalServerError().json(response))
        }
    }
}

/// 获取期货现货价格日线数据（日期范围）
/// GET /futures/spot_price_daily?start_date=20240101&end_date=20240105&symbols=RB,CU
/// 对应 akshare 的 futures_spot_price_daily()
pub async fn get_spot_price_daily(query: web::Query<FuturesSpotPriceDailyQuery>) -> Result<HttpResponse> {
    let symbols: Option<Vec<&str>> = query.symbols.as_ref()
        .map(|s| s.split(',').map(|x| x.trim()).collect());
    
    match get_futures_spot_price_daily(&query.start_date, &query.end_date, symbols).await {
        Ok(data) => {
            let response = ApiResponse::success(data);
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            let response = ApiResponse::<Vec<FuturesSpotPrice>>::error(e.to_string());
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
            .route("/batch", web::post().to(get_multiple_futures))
            // 交易费用和手续费
            .route("/fees", web::get().to(get_fees_info))
            .route("/comm_info", web::get().to(get_comm_info))
            .route("/rule", web::get().to(get_rule))
            // 99期货网库存数据
            .route("/inventory99", web::get().to(get_inventory99))
            .route("/inventory99/symbols", web::get().to(get_inventory99_symbols))
            // 现货价格及基差
            .route("/spot_price", web::get().to(get_spot_price))
            .route("/spot_price_previous", web::get().to(get_spot_price_previous))
            .route("/spot_price_daily", web::get().to(get_spot_price_daily))
            // 持仓排名表与汇总
            .route("/rank/shfe", web::get().to(get_rank_shfe))
            .route("/rank/cffex", web::get().to(get_rank_cffex))
            .route("/rank/dce", web::get().to(get_rank_dce))
            .route("/rank/czce", web::get().to(get_rank_czce))
            .route("/rank/gfex", web::get().to(get_rank_gfex))
            .route("/rank/sum", web::get().to(get_rank_sum_data))
            .route("/rank/sum_daily", web::get().to(get_rank_sum_daily_data))
            // 仓单日报
            .route("/warehouse/czce", web::get().to(get_warehouse_czce))
            .route("/warehouse/dce", web::get().to(get_warehouse_dce))
            .route("/warehouse/shfe", web::get().to(get_warehouse_shfe))
            .route("/warehouse/gfex", web::get().to(get_warehouse_gfex))
            // 主力连续合约
            .route("/main/display", web::get().to(get_display_main_contracts))
            .route("/main/{symbol}/daily", web::get().to(get_main_daily))
            .route("/main/{exchange}", web::get().to(get_main_contracts))
            // 持仓排名
            .route("/hold_pos", web::get().to(get_hold_pos))
            // 外盘期货
            .route("/foreign/symbols", web::get().to(get_foreign_symbols))
            .route("/foreign/realtime", web::post().to(get_foreign_realtime))
            .route("/foreign/{symbol}/history", web::get().to(get_foreign_history))
            .route("/foreign/{symbol}/detail", web::get().to(get_foreign_detail))
            // 品种实时数据
            .route("/realtime/{symbol}", web::get().to(get_realtime_by_symbol))
            // 单个合约
            .route("/{symbol}", web::get().to(get_futures_info))
            .route("/{symbol}/history", web::get().to(get_history))
            .route("/{symbol}/minute", web::get().to(get_minute))
            .route("/{symbol}/detail", web::get().to(get_contract_detail))
    );
}
