//! HTTP 请求处理器模块
//! 
//! 包含所有 API 端点的处理函数

pub mod stock;    // 股票相关接口
pub mod futures;  // 期货相关接口
pub mod health;   // 健康检查接口

use actix_web::web;

/// 配置所有 API 路由
/// 
/// 所有接口统一使用 /api/v1 前缀
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .configure(health::config)   // 健康检查: /api/v1/health
            .configure(stock::config)    // 股票接口: /api/v1/stocks
            .configure(futures::config)  // 期货接口: /api/v1/futures
    );
}