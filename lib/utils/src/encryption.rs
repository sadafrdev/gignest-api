use argon2::password_hash::{PasswordHasher, SaltString, rand_core::OsRng};
use argon2::{Argon2, PasswordVerifier, password_hash::PasswordHash};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use rand::{Rng, distributions::Alphanumeric, thread_rng};
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::json;
use time::{Duration, OffsetDateTime};
use crate::ENV;
use crate::error::AppError;
pub fn verify_password(db_password: &str, password: String) -> Result<(), AppError> {
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

pub async fn send_email(email: &String, otp: String, sendgrid_api_key: &str, from_email: &str) {
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
        .bearer_auth(sendgrid_api_key)
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

#[derive(Clone, Serialize, Deserialize)]
pub struct Env {
    jwt_secret: String,
}

pub fn encoding<T: Serialize>(claims: T) -> Result<String, AppError> {
    let env_var: Env = ENV::load();
    let secret = env_var.jwt_secret;

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|_| AppError::InternalServerError)
}

pub fn decoding<T: DeserializeOwned>(token: &str) -> Result<T, AppError> {
    let env_var: Env = ENV::load();
    let secret = env_var.jwt_secret;

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

    pub async fn verify_reset_token(token: &str) -> Result<ResetTokenClaims, AppError> {
        let data = decoding::<ResetTokenClaims>(token)?;

        if data.sub != "password_reset" {
            return Err(AppError::ValidationError("TOKEN"));
        }

        Ok(data)
    }
}
