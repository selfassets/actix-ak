//! AkShare 后端服务
//! 
//! 提供期货和股票数据的 RESTful API 服务
//! 数据来源：新浪财经、100ppi、99期货网等

mod config;     // 配置模块
mod handlers;   // HTTP 请求处理器
mod middleware; // 中间件
mod models;     // 数据模型定义
mod services;   // 业务逻辑服务

use actix_web::{App, HttpServer, middleware::Logger};
use env_logger::Env;

use crate::config::AppConfig;
use crate::middleware::ApiKeyMiddleware;

/// 应用程序入口
/// 
/// 启动 HTTP 服务器，配置从 config.json 加载
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 加载配置
    let config = AppConfig::load();
    
    // 初始化日志系统
    env_logger::init_from_env(Env::default().default_filter_or(&config.log.level));

    log::info!("启动 AkShare 后端服务");
    log::info!("监听地址: {}", config.bind_addr());

    let api_key = config.api.api_key.clone();
    let bind_addr = config.bind_addr();
    let workers = config.server.workers;

    // 创建并启动 HTTP 服务器
    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(ApiKeyMiddleware::new(api_key.clone()))
            .configure(handlers::config)
    });

    if workers > 0 {
        server = server.workers(workers);
    }

    server.bind(&bind_addr)?.run().await
}