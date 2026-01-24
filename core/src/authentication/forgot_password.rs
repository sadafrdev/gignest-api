use axum::{Json, http::StatusCode};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode, errors::Error};
use core::AppState;
use rand;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};
use sqlx::{postgres::PgRow, FromRow};
use time::{Duration, OffsetDateTime};
use crate::DB;

#[derive(Deserialize, Debug, Serialize, FromRow)]
pub struct SendOtp {
    pub email: String,
}

pub async fn send_email(email: &String, otp: u32) {
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
        .map_err(|e| {
            eprintln!("Email sending error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        });
}

impl SendOtp {
    pub async fn send(self, db: DB) -> Result<(), StatusCode> {
        let otp = rand::random::<u32>() % 1_000_000;
        let otp_hash = format!("{:x}", Sha256::digest(otp.to_string().as_bytes())).clone();

        let email = self.email.clone();

        let user = sqlx::query!(
            r#"
                SELECT email FROM users WHERE email = $1
            "#,
            self.email
        )
        .fetch_optional(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        if user.is_none() {
            println!("Your Email Does Not Exists.");
            return Err(StatusCode::NOT_FOUND);
        }

        sqlx::query!(
            "
                INSERT INTO otps (email, otp_hash, purpose, created_at, expires_at)
                VALUES (
                    $1, $2, 'password_reset', NOW(), NOW() + INTERVAL '10 minutes'
                )
            ",
            self.email,
            self.otp_hash,
        )
        .execute(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        send_email(&email, otp).await;

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResetTokenClaims {
    pub sub: String,
    pub email: String,
    pub iat: i64,
    pub exp: i64,
}

impl ResetTokenClaims{
    pub fn generate_reset_token(email: &str) -> Result<String, jsonwebtoken::errors::Error> {
        let secret = std::env::var("JWT_RESET_SECRET").expect("JWT_RESET_SECRET not set");

        let now = OffsetDateTime::now_utc().unix_timestamp();
        let exp = (OffsetDateTime::now_utc() + Duration::minutes(10)).unix_timestamp();

        let claims = self {
            sub: "password_reset".to_string(),
            email: email.to_string(),
            iat: now,
            exp,
        };

        encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
    }

    pub fn verify_reset_token(token: &str) -> Result<self, Error> {
        let secret = std::env::var("JWT_RESET_SECRET").expect("JWT_RESET_SECRET not set");

        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;

        let data = decode::<self>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &validation,
        )?;

        if data.claims.sub != "password_reset" {
            return Err(jsonwebtoken::errors::Error::from(
                jsonwebtoken::errors::ErrorKind::InvalidToken,
            ));
        }

        Ok(data.claims)
    }

}

#[derive(Deserialize, Debug, Serialize, FromRow)]
pub struct VerifyOtp {
    pub otp: i32,
    pub email: String,
}

impl VerifyOtp{
    pub async fn verify(self, db:DB) -> Result<Json<Value>, StatusCode> {
        let otp_str = format!("{:06}", self.otp);
        let otp_hash = format!("{:x}", Sha256::digest(otp_str.as_bytes()));

        let res = sqlx::query!(
            r#"
                SELECT email
                FROM otps
                WHERE email = $1
                    AND otp_hash = $2
                    AND purpose = 'password_reset'
                    AND expires_at > now()
            "#,
            self.email,
            self.otp_hash
        )
        .fetch_optional(&db)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

        if res.is_none() {
            return Err(StatusCode::UNAUTHORIZED);
        }

        sqlx::query!(
            r#"
                DELETE FROM otps
                WHERE email = $1
                AND purpose = 'password_reset'
            "#,
            self.email
        )
        .execute(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let reset_token = ResetTokenClaims.generate_reset_token(&email).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        return Ok(Json(json!({
            "reset_token": reset_token
        })));
    }
}

#[derive(Deserialize, Debug, Serialize, FromRow)]
pub struct UpdatePassword {
    pub email: String,
    pub new_password: String,
    pub token: String,
}

impl UpdatePassword{
    pub async fn update(self, db: DB) -> Result<Json<Value>, StatusCode> {
        //VErifying Token
        ResetTokenClaims.verify_reset_token(&self.token).map_err(|_| StatusCode::UNAUTHORIZED)?;

        //Updating Password
        sqlx::query!(
            "
                UPDATE users
                SET password = $1
                WHERE email = $2

            ",
            self.new_password,
            self.email
        )
        .execute(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json(json!({
            "message": "Password updated successfully"
        })))
    }
}