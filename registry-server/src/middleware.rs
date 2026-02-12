//! JWT 认证中间件
//!
//! 验证请求中的 Authorization: Bearer <JWT> 头

use crate::auth;
use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use futures::future::{ok, LocalBoxFuture, Ready};
use std::rc::Rc;

/// JWT 认证中间件
pub struct JwtAuth {
    jwt_secret: Rc<String>,
}

impl JwtAuth {
    pub fn new(jwt_secret: String) -> Self {
        Self {
            jwt_secret: Rc::new(jwt_secret),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for JwtAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = JwtAuthService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtAuthService {
            service: Rc::new(service),
            jwt_secret: self.jwt_secret.clone(),
        })
    }
}

pub struct JwtAuthService<S> {
    service: Rc<S>,
    jwt_secret: Rc<String>,
}

impl<S, B> Service<ServiceRequest> for JwtAuthService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        let jwt_secret = self.jwt_secret.clone();

        Box::pin(async move {
            let path = req.path().to_string();

            // 白名单路径直接放行
            if path == "/" || path.starts_with("/api/v1/auth/") {
                let res = service.call(req).await?;
                return Ok(res.map_into_left_body());
            }

            // 提取并验证 Bearer Token
            let token = req
                .headers()
                .get("Authorization")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.strip_prefix("Bearer "));

            match token {
                Some(token) => match auth::validate_token(token, &jwt_secret) {
                    Ok(_claims) => {
                        let res = service.call(req).await?;
                        Ok(res.map_into_left_body())
                    }
                    Err(_) => {
                        let response = HttpResponse::Unauthorized().json(serde_json::json!({
                            "success": false,
                            "message": "Token 无效或已过期",
                            "data": null
                        }));
                        Ok(req.into_response(response).map_into_right_body())
                    }
                },
                None => {
                    let response = HttpResponse::Unauthorized().json(serde_json::json!({
                        "success": false,
                        "message": "缺少认证 Token，请先登录",
                        "data": null
                    }));
                    Ok(req.into_response(response).map_into_right_body())
                }
            }
        })
    }
}
