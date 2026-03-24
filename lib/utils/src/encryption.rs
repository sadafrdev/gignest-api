use argon2::{Argon2, PasswordVerifier, password_hash::PasswordHash};
use argon2::{
    password_hash::{SaltString, PasswordHasher, rand_core::OsRng}
};
use dotenvy::dotenv;
use rand::{Rng, distributions::Alphanumeric, thread_rng};
use reqwest::Client;
use time::{Duration, OffsetDateTime};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde::de::DeserializeOwned;
use crate::error::AppError;

pub fn verify_password(db_password: &str, password: String ) -> Result<(), AppError>{
    let parsed_hash = PasswordHash::new(db_password).map_err(|_| AppError::InternalServerError)?;

    Argon2::default()    
    .verify_password(password.as_bytes(), &parsed_hash)
    .map_err(|_| AppError::ValidationError("Invalid credentials"))?;

    Ok(())
}

pub fn otp() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect()
}

pub fn hashing(pswd: String) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(pswd.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

pub async fn send_email(email: &String, otp: String) {
    dotenv().ok();
    let api_key = std::env::var("SENDGRID_API_KEY").expect("SENDGRID_API_KEY not set");

    let from_email = std::env::var("FROM_EMAIL").expect("FROM_EMAIL not set");

    let client = Client::new();

    let body = json!({
        "personalizations": [{
            "to": [{ "email": email}]
        }],
        "from": { "email": from_email},
        "subject": "Forgot Password OTP",
        "content": [{
            "type": "text/plain",
            "value": format!("Your OTP is {}", otp)
        }]
    });

    let res = client
        .post("https://api.sendgrid.com/v3/mail/send")
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .await
        .inspect_err(|e| eprintln!("Email sending error: {:?}", e))
        .map_err(|_| AppError::InternalServerError);

    match res {
        Ok(response) => {
            println!("Status: {}", response.status());
            let text = response.text().await.unwrap_or_default();
            println!("Body: {}", text);
        }
        Err(e) => {
            eprintln!("Request error: {:?}", e);
        }
    }
}
pub fn encoding<T: Serialize>( claims: T ) -> Result<String, AppError>{
    dotenv::dotenv().ok();
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET not set");

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|_| AppError::InternalServerError)
}

pub fn decoding<T: DeserializeOwned>( token: &str ) -> Result<T, AppError>{
    dotenv::dotenv().ok();
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET not set");

    let data = decode(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| AppError::Unauthorized)?;

    Ok(data.claims)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResetTokenClaims {
    pub sub: String,
    pub email: String,
    pub iat: i64,
    pub exp: i64,
}

impl ResetTokenClaims {
    pub fn generate_reset_token(email: &str) -> Result<String, AppError> {

        let now = OffsetDateTime::now_utc().unix_timestamp();
        let exp = (OffsetDateTime::now_utc() + Duration::minutes(10)).unix_timestamp();

        let claims = ResetTokenClaims {
            sub: "password_reset".to_string(),
            email: email.to_string(),
            iat: now,
            exp,
        };

        encoding(claims)

    }

    pub async fn verify_reset_token(
        token: &str,
    ) -> Result<ResetTokenClaims, AppError> {

        let data = decoding::<ResetTokenClaims>(token)?;

        if data.sub != "password_reset" {
            return Err(AppError::ValidationError("TOKEN"));
        }

        Ok(data)
    }
}
