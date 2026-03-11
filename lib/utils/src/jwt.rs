use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use chrono::{Utc, Duration};
use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub exp: usize,
}

pub fn create_jwt(user_id: i64) -> Result<String, AppError> {
    let secret = std::env::var("JWT_SECRET").map_err(|_| AppError::InternalServerError)?;
    
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .ok_or(AppError::InternalServerError)?
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        exp: expiration,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|_| AppError::InternalServerError)?;

    Ok(token)
}

pub fn verify_jwt(token: &str) -> Result<Claims, AppError> {
    let secret = std::env::var("JWT_SECRET").map_err(|_| AppError::InternalServerError)?;

    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    ).map_err(|_| AppError::Unauthorized)?;

    Ok(data.claims)
}