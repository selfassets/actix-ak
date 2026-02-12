//! 注册中心服务
//!
//! 独立运行的服务注册中心，接收服务注册和心跳

mod auth;
mod config;
mod dashboard;
mod handlers;
mod middleware;
mod models;
mod registry;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger::Env;

use crate::auth::UserStore;
use crate::config::AppConfig;
use crate::middleware::JwtAuth;
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

    // 创建用户存储并初始化管理员账号
    let user_store = UserStore::new();
    if let Err(e) = user_store
        .add_user(
            &config.auth.admin_username,
            &config.auth.admin_password,
            "admin",
        )
        .await
    {
        log::error!("初始化管理员账号失败: {}", e);
    } else {
        log::info!("管理员账号已初始化: {}", config.auth.admin_username);
    }

    let registry_data = web::Data::new(registry);
    let user_store_data = web::Data::new(user_store);
    let auth_config_data = web::Data::new(config.auth.clone());
    let jwt_secret = config.auth.jwt_secret.clone();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(registry_data.clone())
            .app_data(user_store_data.clone())
            .app_data(auth_config_data.clone())
            .wrap(Logger::default())
            .wrap(cors)
            .wrap(JwtAuth::new(jwt_secret.clone()))
            .configure(handlers::config)
    })
    .bind(&bind_addr)?
    .run()
    .await
}
