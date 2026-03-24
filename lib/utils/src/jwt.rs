use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};
use crate::{encryption::{encoding, decoding}, error::AppError};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub exp: usize,
}

pub fn create_jwt(user_id: i64) -> Result<String, AppError> {
    
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .ok_or(AppError::InternalServerError)?
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        exp: expiration,
    };

   encoding(claims)
}

pub fn verify_jwt(token: &str) -> Result<Claims, AppError> {
    decoding::<Claims>(token)
}
