use crate::models::jwt::Claims;
use crate::utils::Config;
use chrono::{Duration, Utc};
use dotenv::dotenv;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};

pub fn create_jwt_token(username: &str, _password: &str) -> String {
    dotenv().ok();
    let expire = (Utc::now() + Duration::hours(1)).timestamp() as usize;
    let claims = Claims {
        sub: username.to_owned(),
        exp: expire,
    };
    let config = Config::from_env();
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_ref()),
    )
    .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Token creation error: {}", e)))
    .unwrap()
}

pub fn validate_jwt_token(
    token: &str,
    jwt_secret_key: &str,
) -> actix_web::Result<TokenData<Claims>, actix_web::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret_key.as_ref()),
        &Validation::default(),
    )
    .map_err(|e| actix_web::error::ErrorUnauthorized(format!("Invalid token: {}", e)))
}
