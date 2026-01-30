use std::future::{Ready, ready};
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse, web,
    body::{EitherBody},
};
use futures_util::future::LocalBoxFuture;
use crate::auth::jwt::validate_jwt_token;
use crate::models::app_state::AppState;

pub struct JwtAuth;

impl<S, B> Transform<S, ServiceRequest> for JwtAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = JwtAuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtAuthMiddleware { service }))
    }
}

pub struct JwtAuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for JwtAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth_header = req.headers().get("Authorization");

        let auth_token = match auth_header {
            Some(h) => {
                let s = h.to_str().unwrap_or("");
                if s.starts_with("Bearer ") {
                    Some(s[7..].to_string())
                } else {
                    None
                }
            }
            None => None,
        };

        if let Some(token) = auth_token {
            let state = req.app_data::<web::Data<AppState>>().unwrap();
            
            match validate_jwt_token(&token, &state.jwt_secret_key) {
                Ok(_) => {
                    let fut = self.service.call(req);
                    Box::pin(async move {
                        let res = fut.await?;
                        Ok(res.map_into_left_body())
                    })
                }
                Err(e) => {
                    let (request, _pl) = req.into_parts();
                    let response = HttpResponse::Unauthorized()
                        .body(format!("Unauthorized: {}", e))
                        .map_into_right_body();
                    Box::pin(async move { Ok(ServiceResponse::new(request, response)) })
                }
            }
        } else {
            let (request, _pl) = req.into_parts();
            let response = HttpResponse::Unauthorized()
                .body("Unauthorized: Missing or invalid token")
                .map_into_right_body();
            Box::pin(async move { Ok(ServiceResponse::new(request, response)) })
        }
    }
}
