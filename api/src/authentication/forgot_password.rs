use axum::{
    Json, Router,
    extract::{Extension, State},
    routing::{patch, post},
};
use serde::Deserialize;
use serde_json::Value;
use utils::{db::DB, error::AppError};
use authentication::forgot_password::{SendOtp, UpdatePassword, VerifyOtp};

pub async fn send_otp(
    State(env): State<ENV>,
    Extension(db): Extension<DB>,
    Json(form): Json<SendOtp>,
) -> Result<(), AppError> {
    form.send_otp(db, &env.sendgrid_api_key, &env.from_email).await
}

pub async fn verify_otp(
    Extension(db): Extension<DB>,
    Json(form): Json<VerifyOtp>,
) -> Result<Json<Value>, AppError> {
    form.verify_otp(db).await
}

pub async fn update_password(
    Extension(db): Extension<DB>,
    Json(form): Json<UpdatePassword>,
) -> Result<Json<Value>, AppError> {
    form.update_password(db).await
}

#[derive(Deserialize, Clone)]
pub struct ENV{
    from_email: String,
    sendgrid_api_key: String
}

pub fn router() -> Router {
    let env: ENV = utils::ENV::load();
    Router::new()
        .route("/forgot-password/send-otp", post(send_otp))
        .route("/forgot-password/verify-otp", post(verify_otp))
        .route("/forgot-password/update-password", patch(update_password))
        .with_state(env)
}
