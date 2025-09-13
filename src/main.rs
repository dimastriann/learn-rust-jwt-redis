mod auth;
mod models;
mod routes;
mod utils;

use crate::models::jwt::AppState;
use crate::utils::Config;
use actix_web::{App, HttpServer, web};
use log::info;
use redis::Client;
use dotenv::dotenv;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Starting JWT-only example at http://127.0.0.1:8080");
    
    dotenv().ok();

    // TODO: move to reusable file
    // Setup redis connection
    let config = Config::from_env();
    let client = Client::open(config.redis_url.as_str())
        .expect("Failed to connect to redis");
    let app_state = web::Data::new(AppState {
        redis_client: client,
        jwt_secret_key: config.jwt_secret
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(routes::hello::hello_routes)
            .configure(routes::jwt_controller::jwt_routes)
            .configure(routes::jwt_redis_controller::jwt_redis_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
