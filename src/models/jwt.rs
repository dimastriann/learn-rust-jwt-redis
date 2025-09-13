use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub struct AppState {
    pub redis_client: redis::Client,
    pub jwt_secret_key: String,
}
