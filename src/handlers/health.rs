use actix_web::{web, HttpResponse, Result};
use crate::models::ApiResponse;

pub async fn health_check() -> Result<HttpResponse> {
    let response = ApiResponse::success("Service is healthy");
    Ok(HttpResponse::Ok().json(response))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check));
}