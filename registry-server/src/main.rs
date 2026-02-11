//! 注册中心服务
//!
//! 独立运行的服务注册中心，接收服务注册和心跳

mod config;
mod dashboard;
mod handlers;
mod models;
mod registry;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger::Env;

use crate::config::AppConfig;
use crate::registry::ServiceRegistry;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 加载配置
    let config = AppConfig::load();

    // 初始化日志
    env_logger::init_from_env(Env::default().default_filter_or(&config.log.level));

    log::info!("启动注册中心服务");
    log::info!("监听地址: {}", config.bind_addr());
    log::info!("心跳超时: {}s", config.registry.heartbeat_timeout_secs);

    let bind_addr = config.bind_addr();

    // 创建注册表
    let registry = ServiceRegistry::new(config.registry.heartbeat_timeout_secs);

    // 启动过期清理定时任务
    registry.clone().start_cleanup_task();

    let registry_data = web::Data::new(registry);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(registry_data.clone())
            .wrap(Logger::default())
            .wrap(cors)
            .configure(handlers::config)
    })
    .bind(&bind_addr)?
    .run()
    .await
}
