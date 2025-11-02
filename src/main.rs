mod auth;
mod models;
mod routes;
mod utils;
mod db;
mod schema;

use crate::models::app_state::AppState;
use crate::utils::Config;
use actix_web::{App, HttpServer, web, middleware::Logger};
use redis::Client;
use dotenv::dotenv;
use crate::db::establish_db_connection;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // access logs are printed with the INFO level so ensure it is enabled by default
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // TODO: move to reusable file
    // Setup redis connection
    let config = Config::from_env();
    let client = Client::open(config.redis_url.as_str())
        .expect("Failed to connect to redis");
    let app_state = web::Data::new(AppState {
        redis_client: client,
        jwt_secret_key: config.jwt_secret,
        pool: establish_db_connection(),
    });

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(app_state.clone())
            .configure(routes::hello::hello_routes)
            .configure(routes::jwt_controller::jwt_routes)
            .configure(routes::jwt_redis_controller::jwt_redis_routes)
            .configure(routes::product::product_routes)
            .configure(routes::product::category_routes)
            .configure(routes::user_contact::contact_routes)
            .configure(routes::user_contact::user_routes)
            .configure(routes::order::order_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
