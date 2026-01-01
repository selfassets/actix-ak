//! API Key 认证中间件
//!
//! 通过 Header 中的 Authorization: Bearer <token> 进行认证

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
    body::EitherBody,
};
use futures::future::{ok, LocalBoxFuture, Ready};
use std::rc::Rc;

/// API Key 中间件
pub struct ApiKeyMiddleware {
    api_key: Rc<String>,
}

impl ApiKeyMiddleware {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key: Rc::new(api_key),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for ApiKeyMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = ApiKeyMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(ApiKeyMiddlewareService {
            service: Rc::new(service),
            api_key: self.api_key.clone(),
        })
    }
}

pub struct ApiKeyMiddlewareService<S> {
    service: Rc<S>,
    api_key: Rc<String>,
}

impl<S, B> Service<ServiceRequest> for ApiKeyMiddlewareService<S>
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
        let api_key = self.api_key.clone();

        Box::pin(async move {
            // 跳过健康检查接口
            if req.path().ends_with("/health") {
                let res = service.call(req).await?;
                return Ok(res.map_into_left_body());
            }

            // 验证 Bearer Token
            let provided_key = req
                .headers()
                .get("Authorization")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.strip_prefix("Bearer "));

            match provided_key {
                Some(key) if key == api_key.as_str() => {
                    let res = service.call(req).await?;
                    Ok(res.map_into_left_body())
                }
                _ => {
                    let response = HttpResponse::Unauthorized()
                        .json(serde_json::json!({
                            "code": 401,
                            "message": "无效的 Bearer Token",
                            "data": null
                        }));
                    Ok(req.into_response(response).map_into_right_body())
                }
            }
        })
    }
}
