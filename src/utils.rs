use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub jwt_secret: String,
    pub redis_url: String,
    pub use_redis_cache: bool,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();

        let jwt_secret = env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set");
        let redis_url = env::var("REDIS_URL").unwrap_or("redis://127.0.0.1/".to_string());
        let use_redis_cache = env::var("USE_REDIS_CACHE").is_ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        Self {
            jwt_secret,
            redis_url,
            use_redis_cache,
            database_url,
        }
    }
}
