//! AkShare 后端服务
//! 
//! 提供期货和股票数据的 RESTful API 服务
//! 数据来源：新浪财经、100ppi、99期货网等

mod handlers;   // HTTP 请求处理器
mod middleware; // 中间件
mod models;     // 数据模型定义
mod services;   // 业务逻辑服务

use actix_web::{App, HttpServer, middleware::Logger};
use env_logger::Env;
use std::env;

use crate::middleware::ApiKeyMiddleware;

/// 应用程序入口
/// 
/// 启动 HTTP 服务器，监听 127.0.0.1:8080
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化日志系统，默认日志级别为 info
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // 从环境变量获取 API Key
    let api_key = env::var("API_KEY").unwrap_or_else(|_| {
        log::warn!("未设置 API_KEY 环境变量，使用默认值");
        "default-api-key".to_string()
    });

    log::info!("启动 AkShare 后端服务");

    // 创建并启动 HTTP 服务器
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())  // 添加请求日志中间件
            .wrap(ApiKeyMiddleware::new(api_key.clone()))  // API Key 认证
            .configure(handlers::config)  // 配置路由
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}