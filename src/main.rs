use actix_web::{get, route, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use chrono::{Utc, Duration};
use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, TokenData};
use log::info;

// TODO: get from env file
const JWT_SECRET: &str = "dimastriann.rust.rest.api";

// Struct
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Debug, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct LoginResponse {
    token: String,
    expires_at: String,
}

fn validate_jwt_token(token: &str) -> Result<TokenData<Claims>, actix_web::Error> {
    decode::<Claims>(
        token,
         &DecodingKey::from_secret(JWT_SECRET.as_ref()), 
        &Validation::default(),
    ).map_err(|e|
        actix_web::error::ErrorUnauthorized(format!("Invalid token: {}", e))
    )
}

#[get("/")]
async fn hello() -> impl Responder {
    format!("Hello World!")
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[route("/login", method="POST")]
async fn login(data: web::Json<LoginRequest>) -> Result<impl Responder> {
    // hardcode payload 
    if data.username != "admin" || data.password != "password" {
        return Ok(HttpResponse::Unauthorized().body("Invalid username or password !"));
    }

    let expire = (Utc::now() + Duration::hours(1)).timestamp() as usize;
    let claims = Claims {
        sub: data.username.clone(),
        exp: expire,
    };
    let token = encode(
        &Header::default(), 
        &claims, 
        &EncodingKey::from_secret(JWT_SECRET.as_ref())
    ).map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Token creation error: {}", e))
    })?;
    let response = LoginResponse {
        token,
        expires_at: (Utc::now() + Duration::hours(1)).to_rfc3339(),
    };
    Ok(HttpResponse::Ok().json(response))
}

#[route("/protected-route", method="GET")]
async fn protected(req: HttpRequest) -> Result<impl Responder> {
    // Check Authorization header
    println!("Headers: {:?}", req.headers());
    let auth_header = match req.headers().get("Authorization") {
        Some(h) => h.to_str().map_err(|_| actix_web::error::ErrorUnauthorized("Invalid Authorization Header"))?,
        None => return Ok(HttpResponse::Unauthorized().body("Missing Authorization")),
    };

    // Check bearer header
    if !auth_header.starts_with("Bearer ") {
        return Ok(HttpResponse::Unauthorized().body("Invalid Authorization header"));
    }

    let token = &auth_header[7..];
    let token_data = validate_jwt_token(token)?;

    Ok(HttpResponse::Ok().body(format!("Hello, {}! This is protected data.", token_data.claims.sub)))
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Starting JWT-only example at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(greet)
            .service(login)
            .service(protected)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
