//! 注册中心 HTTP 接口

use crate::dashboard;
use crate::models::{ApiResponse, HeartbeatRequest, RegisterRequest, RegisterResponse};
use crate::registry::ServiceRegistry;
use actix_web::{web, HttpResponse, Result};
use uuid::Uuid;

/// POST /api/v1/registry/register
pub async fn register(
    registry: web::Data<ServiceRegistry>,
    body: web::Json<RegisterRequest>,
) -> Result<HttpResponse> {
    let req = body.into_inner();
    let instance_id = Uuid::new_v4().to_string();

    registry
        .register(
            instance_id.clone(),
            req.service_name,
            req.host,
            req.port,
            req.metadata,
        )
        .await;

    let response = ApiResponse::success(RegisterResponse { instance_id });
    Ok(HttpResponse::Ok().json(response))
}

/// DELETE /api/v1/registry/deregister/{instance_id}
pub async fn deregister(
    registry: web::Data<ServiceRegistry>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let instance_id = path.into_inner();

    if registry.deregister(&instance_id).await {
        let response: ApiResponse<String> = ApiResponse::success("注销成功".to_string());
        Ok(HttpResponse::Ok().json(response))
    } else {
        let response: ApiResponse<String> =
            ApiResponse::error(format!("实例不存在: {}", instance_id));
        Ok(HttpResponse::NotFound().json(response))
    }
}

/// POST /api/v1/registry/heartbeat
pub async fn heartbeat(
    registry: web::Data<ServiceRegistry>,
    body: web::Json<HeartbeatRequest>,
) -> Result<HttpResponse> {
    let req = body.into_inner();

    if registry.heartbeat(&req.instance_id).await {
        let response: ApiResponse<String> = ApiResponse::success("心跳接收成功".to_string());
        Ok(HttpResponse::Ok().json(response))
    } else {
        let response: ApiResponse<String> =
            ApiResponse::error(format!("实例不存在: {}", req.instance_id));
        Ok(HttpResponse::NotFound().json(response))
    }
}

/// GET /api/v1/registry/instances
pub async fn get_instances(registry: web::Data<ServiceRegistry>) -> Result<HttpResponse> {
    let instances = registry.get_instances().await;
    let response = ApiResponse::success(instances);
    Ok(HttpResponse::Ok().json(response))
}

/// GET / — 仪表板页面
pub async fn dashboard_page() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(dashboard::dashboard_html()))
}

/// 配置注册中心路由
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(dashboard_page)).service(
        web::scope("/api/v1/registry")
            .route("/register", web::post().to(register))
            .route("/deregister/{instance_id}", web::delete().to(deregister))
            .route("/heartbeat", web::post().to(heartbeat))
            .route("/instances", web::get().to(get_instances)),
    );
}
