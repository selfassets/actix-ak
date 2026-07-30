//! 加密货币数据 HTTP 处理器

use crate::models::{ak::crypto::CryptoQuery, ApiResponse};
use crate::services::ak::crypto;
use actix_web::{web, HttpResponse, Result};

/// 获取芝加哥商业交易所 (CME) 比特币成交量及持仓报告
///
/// GET /api/v1/ak/crypto/bitcoin_cme
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/crypto/bitcoin_cme",
        tag = "crypto",
        params(
            CryptoQuery
        ),
        responses(
            (status = 200, description = "成功获取 CME 比特币成交量及持仓报告", body = ApiResponse<Vec<CryptoBitcoinCmeItem>>)
        )
    )
)]
pub async fn get_crypto_bitcoin_cme(query: web::Query<CryptoQuery>) -> Result<HttpResponse> {
    match crypto::get_crypto_bitcoin_cme(query.into_inner()).await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 获取全球机构/上市公司比特币持仓报告
///
/// GET /api/v1/ak/crypto/bitcoin_hold_report
#[cfg_attr(
    feature = "swagger",
    utoipa::path(
        get,
        path = "/ak/crypto/bitcoin_hold_report",
        tag = "crypto",
        responses(
            (status = 200, description = "成功获取机构比特币持仓报告", body = ApiResponse<Vec<CryptoBitcoinHoldItem>>)
        )
    )
)]
pub async fn get_crypto_bitcoin_hold_report() -> Result<HttpResponse> {
    match crypto::get_crypto_bitcoin_hold_report().await {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err))),
    }
}

/// 配置加密货币路由
///
/// 挂载路径：/crypto
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/crypto")
            .route("/bitcoin_cme", web::get().to(get_crypto_bitcoin_cme))
            .route(
                "/bitcoin_hold_report",
                web::get().to(get_crypto_bitcoin_hold_report),
            ),
    );
}
