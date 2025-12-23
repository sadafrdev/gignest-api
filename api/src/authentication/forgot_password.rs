use rand;
use axum::{Json, extract::State, http::StatusCode,};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use reqwest::Client;
use serde_json::json;
use sqlx::prelude::FromRow;
use crate::AppState;
use sqlx::postgres::PgRow;

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

pub async fn request_password_reset(
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
