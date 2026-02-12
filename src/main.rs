//! AkShare 后端服务
//!
//! 提供期货和股票数据的 RESTful API 服务
//! 数据来源：新浪财经、100ppi、99期货网等

mod config; // 配置模块
mod handlers; // HTTP 请求处理器
mod middleware; // 中间件
mod models; // 数据模型定义
mod services; // 业务逻辑服务

use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer};
use env_logger::Env;

use crate::config::AppConfig;
use crate::middleware::ApiKeyMiddleware;
use crate::services::registry_client::RegistryClient;

// Swagger 相关导入（仅在启用 swagger feature 时编译）
#[cfg(feature = "swagger")]
use utoipa::OpenApi;
#[cfg(feature = "swagger")]
use utoipa_swagger_ui::SwaggerUi;

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

    #[cfg(feature = "swagger")]
    log::info!(
        "Swagger UI 已启用: http://{}/swagger-ui/",
        config.bind_addr()
    );

    let api_key = config.api.api_key.clone();
    let bind_addr = config.bind_addr();
    let workers = config.server.workers;

    // 配置开启注册且注册中心地址非空时，启动客户端注册和心跳任务
    if config.registry.enabled && !config.registry.registry_url.is_empty() {
        log::info!(
            "注册中心客户端已启用，目标: {}",
            config.registry.registry_url
        );
        // 获取注册用的 host，优先级：register_host 配置 > 本机 hostname > 127.0.0.1
        let register_host = if !config.registry.register_host.is_empty() {
            config.registry.register_host.clone()
        } else {
            hostname::get()
                .ok()
                .and_then(|h| h.into_string().ok())
                .unwrap_or_else(|| "127.0.0.1".to_string())
        };

        let client = RegistryClient::new(
            config.registry.registry_url.clone(),
            config.registry.service_name.clone(),
            register_host,
            config.server.port,
            config.registry.heartbeat_interval_secs,
            config.registry.registry_username.clone(),
            config.registry.registry_password.clone(),
        );
        client.start_heartbeat_task();
    }

    // 创建并启动 HTTP 服务器
    let mut server = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        let app = App::new()
            .wrap(ApiKeyMiddleware::new(api_key.clone()))
            .wrap(Logger::default())
            .wrap(cors)
            .configure(handlers::config);

        // 条件挂载 Swagger UI（仅在启用 swagger feature 时）
        #[cfg(feature = "swagger")]
        let app = app.service(SwaggerUi::new("/swagger-ui/{_:.*}").url(
            "/api-docs/openapi.json",
            handlers::openapi::ApiDoc::openapi(),
        ));

        app
    });

    if workers > 0 {
        server = server.workers(workers);
    }

    server.bind(&bind_addr)?.run().await
}
