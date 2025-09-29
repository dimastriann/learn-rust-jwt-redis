use crate::auth::jwt::{create_jwt_token, validate_jwt_token};
use crate::models::app_state::AppState;
use crate::models::login::{LoginRequest, LoginResponse};
use actix_web::{HttpRequest, HttpResponse, Responder, route, web};
use chrono::{Duration, Utc};
use dotenv::dotenv;
use redis::TypedCommands;

#[route("/redis-login", method = "POST")]
async fn login(
    data: web::Json<LoginRequest>,
    state: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    dotenv().ok();
    // hardcode payload
    if data.username != "admin" || data.password != "password" {
        return Ok(HttpResponse::Unauthorized().body("Invalid username or password !"));
    }

    let token = create_jwt_token(&data.username, &data.password);

    // Store token to redis
    let mut redis_conn = state.redis_client.get_connection().map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Token creation error: {}", e))
    })?;
    let ttl_seconds = (60 * 5) as u64;
    let _: () = redis_conn
        .set_ex(&token, &data.username, ttl_seconds)
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Token creation error: {}", e))
        })?;

    let response = LoginResponse {
        token,
        expires_at: (Utc::now() + Duration::hours(1)).to_rfc3339(),
    };
    Ok(HttpResponse::Ok().json(response))
}

#[route("/redis-protected-route", method = "GET")]
async fn protected(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    // Check Authorization header
    // println!("Headers: {:?}", req.headers());
    let auth_header = match req.headers().get("Authorization") {
        Some(h) => h
            .to_str()
            .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid Authorization Header"))?,
        None => return Ok(HttpResponse::Unauthorized().body("Missing Authorization")),
    };

    // Check bearer header
    if !auth_header.starts_with("Bearer ") {
        return Ok(HttpResponse::Unauthorized().body("Invalid Authorization header"));
    }

    let token = &auth_header[7..];
    let token_data = validate_jwt_token(token, state.jwt_secret_key.as_ref())?;

    let mut redis_conn = state.redis_client.get_connection().map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Token creation error: {}", e))
    })?;
    let exists: Option<String> = redis_conn.get(token).map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Redis get error: {}", e))
    })?;

    if exists.is_none() {
        return Ok(HttpResponse::ExpectationFailed().body("No token exists or expired"));
    }

    Ok(HttpResponse::Ok().body(format!(
        "Hello, {}! This is redis protected route data.",
        token_data.claims.sub
    )))
}

pub fn jwt_redis_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(login).service(protected);
}
