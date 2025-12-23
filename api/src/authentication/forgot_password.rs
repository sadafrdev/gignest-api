use rand;
use axum::{Json, extract::State, http::StatusCode,};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use reqwest::Client;
use serde_json::json;
use sqlx::prelude::FromRow;
use crate::AppState;
use sqlx::postgres::PgRow;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header, decode, DecodingKey, Validation};
use time::{Duration, OffsetDateTime};

#[derive(Deserialize, Debug, Serialize, FromRow)]
pub struct ResetClaims {
    pub email: String
}

pub async fn send_email(email: &String, otp: u32) {
    let api_key = std::env::var("SENDGRID_API_KEY")
    .expect("SENDGRID_API_KEY not set");

    let from_email = std::env::var("FROM_EMAIL")
    .expect("FROM_EMAIL not set");

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

    println!("{:?}", res);
}

pub async fn send_otp(
    State(state): State<AppState>,
    Json(payload): Json<ResetClaims>,
)-> Result<(), StatusCode>{
    let otp = rand::random::<u32>() % 1_000_000;
    let otp_hash = format!("{:x}", Sha256::digest(otp.to_string().as_bytes())).clone();

    let email= payload.email.clone();

    let user: Option<PgRow> = sqlx::query(
        r#"
            SELECT email FROM users WHERE email = $1
        "#
    )
    .bind(&email)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if user.is_none() {
        println!("Your Email Does Not Exists.");
        return Err(StatusCode::NOT_FOUND);
    }

    sqlx::query(
        "
            INSERT INTO otps (email, otp_hash, purpose, created_at, expires_at)
            VALUES (
                $1, $2, 'password_reset', NOW(), NOW() + INTERVAL '10 minutes'
            )
        "
    )
    .bind(&email)
    .bind(&otp_hash)
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    send_email(&email, otp).await;

    Ok(())
}

#[derive(Deserialize, Debug, Serialize, FromRow)]
pub struct OTPVerficationPayload{
    pub otp: i32,
    pub email: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResetTokenClaims {
    pub sub: String,
    pub email: String,
    pub iat: i64,
    pub exp: i64,
}

pub fn generate_reset_token(email: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = std::env::var("JWT_RESET_SECRET")
        .expect("JWT_RESET_SECRET not set");

    let now = OffsetDateTime::now_utc().unix_timestamp();
    let exp = (OffsetDateTime::now_utc() + Duration::minutes(10)).unix_timestamp();

    let claims = ResetTokenClaims {
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

pub async fn verify_otp(
    State(state): State<AppState>,
    Json(payload): Json<OTPVerficationPayload>
)-> Result<Json<serde_json::Value>, StatusCode>{
    let email = payload.email.clone();

    let otp_str = format!("{:06}", payload.otp);
    let otp_hash = format!("{:x}", Sha256::digest(otp_str.as_bytes()));

    let res= sqlx::query(
        r#"SELECT email
            FROM otps
            WHERE email = $1
                AND otp_hash = $2
                AND purpose = 'password_reset'
                AND expires_at > now()
            "#
    )
    .bind(&email)
    .bind(otp_hash)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| 
        StatusCode::NOT_FOUND
    )?;

    if res.is_none(){
        return Err(StatusCode::UNAUTHORIZED)
    }

    sqlx::query(
        r#"
        DELETE FROM otps
        WHERE email = $1
          AND purpose = 'password_reset'
        "#
    )
    .bind(&email)
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let reset_token = generate_reset_token(&email)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    return Ok(Json(serde_json::json!({
        "reset_token": reset_token
    })));
}

#[derive(Deserialize, Debug, Serialize, FromRow)]
pub struct ResetPasswordPayload{
    pub email: String,
    pub new_password: String,
    pub token: String
}
pub fn verify_reset_token(token: &str) -> Result<ResetTokenClaims, jsonwebtoken::errors::Error> {
    let secret = std::env::var("JWT_RESET_SECRET").expect("JWT_RESET_SECRET not set");

    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;

    let data = decode::<ResetTokenClaims>(
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

pub async fn update_password(
    State(state): State<AppState>,
    Json(payload): Json<ResetPasswordPayload>
) -> Result<Json<serde_json::Value>, StatusCode> {
    //VErifying Token
    verify_reset_token(&payload.token)
        .map_err(|_| StatusCode::UNAUTHORIZED);

    //Updating Password
    sqlx::query(
        "
            UPDATE users
            SET password = $1
            WHERE email = $2

        "
    )
    .bind(payload.new_password)
    .bind(payload.email)
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);

    Ok(Json(json!({
        "message": "Password updated successfully"
    })))
}
