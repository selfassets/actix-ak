mod handlers;
mod models;
mod services;

use actix_web::{App, HttpServer, middleware::Logger};
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    log::info!("Starting AkShare Backend Server");

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(handlers::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}