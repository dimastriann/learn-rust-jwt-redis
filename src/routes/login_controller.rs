// use actix_web::{route, web, App, HttpServer, Responder};
// use serde::{Deserialize, Serialize};

// #[derive(Debug, Serialize, Deserialize)]
// struct Claims {
//     sub: String,
//     exp: usize,
// }

// #[derive(Debug, Deserialize)]
// struct LoginRequest {
//     username: String,
//     password: String,
// }

// #[derive(Debug, Serialize)]
// struct LoginResponse {
//     token: String,
//     expires_at: String,
// }

// const JWT_SECRET: &str = "dimastriann.rust.rest.api"; // Replace with env var in production

// #[route("/login", method="POST")]
// async fn login(paylod: web::Json) -> Result<impl Responder>{
//     Ok(HttpRes)
// }

// fn validate_jwt_token(){}

// async fn protected(){}
