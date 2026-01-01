pub mod stock;
pub mod futures;
pub mod health;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .configure(health::config)
            .configure(stock::config)
            .configure(futures::config)
    );
}