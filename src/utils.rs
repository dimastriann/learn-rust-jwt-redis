use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub jwt_secret: String,
    pub redis_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();

        let jwt_secret = env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set");
        let redis_url = env::var("REDIS_URL").unwrap_or("redis://127.0.0.1/".to_string());

        Self {
            jwt_secret,
            redis_url,
        }
    }
}
