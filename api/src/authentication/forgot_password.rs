use authentication::forgot_password::{
    SendOtp, UpdatePassword, UpdatePasswordResponse, VerifyOtp, VerifyOtpResponse,
};
use axum::{
    Json, Router,
    extract::{Extension, State},
    routing::{patch, post},
};
use serde::Deserialize;
use utils::{db::DB, error::AppError};

pub async fn send_otp(
    State(env): State<ENV>,
    Extension(db): Extension<DB>,
    Json(form): Json<SendOtp>,
) -> Result<(), AppError> {
    form.send_otp(db, &env.sendgrid_api_key, &env.from_email)
        .await
}

pub async fn verify_otp(
    Extension(db): Extension<DB>,
    Json(form): Json<VerifyOtp>,
) -> Result<Json<VerifyOtpResponse>, AppError> {
    form.verify_otp(db).await.map(Json)
}

pub async fn update_password(
    Extension(db): Extension<DB>,
    Json(form): Json<UpdatePassword>,
) -> Result<Json<UpdatePasswordResponse>, AppError> {
    form.update_password(db).await.map(Json)
}

#[derive(Deserialize, Clone)]
pub struct ENV {
    from_email: String,
    sendgrid_api_key: String,
}

pub fn router() -> Router {
    let env: ENV = utils::ENV::load();
    Router::new()
        .route("/forgot-password/send-otp", post(send_otp))
        .route("/forgot-password/verify-otp", post(verify_otp))
        .route("/forgot-password/update-password", patch(update_password))
        .with_state(env)
}
