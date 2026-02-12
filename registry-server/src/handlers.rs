//! 注册中心 HTTP 接口

use crate::auth::{self, UserStore};
use crate::config::AuthConfig;
use crate::dashboard;
use crate::models::{
    ApiResponse, HeartbeatRequest, LoginRequest, LoginResponse, RegisterRequest, RegisterResponse,
    RegisterUserRequest,
};
use crate::registry::ServiceRegistry;
use actix_web::{web, HttpRequest, HttpResponse, Result};
use uuid::Uuid;

/// POST /api/v1/auth/login
pub async fn login(
    user_store: web::Data<UserStore>,
    auth_config: web::Data<AuthConfig>,
    body: web::Json<LoginRequest>,
) -> Result<HttpResponse> {
    let req = body.into_inner();

    match user_store.verify(&req.username, &req.password).await {
        Some(user) => {
            match auth::create_token(
                &user.username,
                &user.role,
                &auth_config.jwt_secret,
                auth_config.token_expire_hours,
            ) {
                Ok(token) => {
                    let response = ApiResponse::success(LoginResponse {
                        token,
                        expires_in: auth_config.token_expire_hours * 3600,
                    });
                    Ok(HttpResponse::Ok().json(response))
                }
                Err(e) => {
                    let response: ApiResponse<String> =
                        ApiResponse::error(format!("Token 生成失败: {}", e));
                    Ok(HttpResponse::InternalServerError().json(response))
                }
            }
        }
        None => {
            let response: ApiResponse<String> = ApiResponse::error("用户名或密码错误".to_string());
            Ok(HttpResponse::Unauthorized().json(response))
        }
    }
}

/// POST /api/v1/auth/register
pub async fn register_user(
    user_store: web::Data<UserStore>,
    body: web::Json<RegisterUserRequest>,
) -> Result<HttpResponse> {
    let req = body.into_inner();

    if req.username.is_empty() || req.password.is_empty() {
        let response: ApiResponse<String> = ApiResponse::error("用户名和密码不能为空".to_string());
        return Ok(HttpResponse::BadRequest().json(response));
    }

    if req.password.len() < 6 {
        let response: ApiResponse<String> = ApiResponse::error("密码长度至少 6 位".to_string());
        return Ok(HttpResponse::BadRequest().json(response));
    }

    match user_store
        .add_user(&req.username, &req.password, "user")
        .await
    {
        Ok(()) => {
            let response: ApiResponse<String> = ApiResponse::success("注册成功".to_string());
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            let response: ApiResponse<String> = ApiResponse::error(e);
            Ok(HttpResponse::Conflict().json(response))
        }
    }
}

/// POST /api/v1/auth/refresh
/// 使用当前有效 Token 换取新 Token
pub async fn refresh_token(
    req: HttpRequest,
    auth_config: web::Data<AuthConfig>,
) -> Result<HttpResponse> {
    // 从请求头提取当前 Token
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .unwrap_or("");

    if token.is_empty() {
        let response: ApiResponse<String> = ApiResponse::error("缺少 Token".to_string());
        return Ok(HttpResponse::Unauthorized().json(response));
    }

    // 验证当前 Token 并提取用户信息
    match auth::validate_token(token, &auth_config.jwt_secret) {
        Ok(claims) => {
            // 用原有用户信息签发新 Token
            match auth::create_token(
                &claims.sub,
                &claims.role,
                &auth_config.jwt_secret,
                auth_config.token_expire_hours,
            ) {
                Ok(new_token) => {
                    let response = ApiResponse::success(LoginResponse {
                        token: new_token,
                        expires_in: auth_config.token_expire_hours * 3600,
                    });
                    Ok(HttpResponse::Ok().json(response))
                }
                Err(e) => {
                    let response: ApiResponse<String> =
                        ApiResponse::error(format!("Token 签发失败: {}", e));
                    Ok(HttpResponse::InternalServerError().json(response))
                }
            }
        }
        Err(_) => {
            let response: ApiResponse<String> =
                ApiResponse::error("Token 已过期或无效".to_string());
            Ok(HttpResponse::Unauthorized().json(response))
        }
    }
}

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
    cfg.route("/", web::get().to(dashboard_page))
        .service(
            web::scope("/api/v1/auth")
                .route("/login", web::post().to(login))
                .route("/register", web::post().to(register_user))
                .route("/refresh", web::post().to(refresh_token)),
        )
        .service(
            web::scope("/api/v1/registry")
                .route("/register", web::post().to(register))
                .route("/deregister/{instance_id}", web::delete().to(deregister))
                .route("/heartbeat", web::post().to(heartbeat))
                .route("/instances", web::get().to(get_instances)),
        );
}
