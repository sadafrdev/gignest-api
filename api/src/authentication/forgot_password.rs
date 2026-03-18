use axum::{
    Json, Router,
    extract::Extension,
    routing::{patch, post},
};
use serde_json::Value;
use sqlx::PgPool;
use utils::{db::AppState, error::AppError};
use authentication::forgot_password::{SendOtp, UpdatePassword, VerifyOtp};

pub async fn send_otp(
    Extension(db): Extension<PgPool>,
    Json(form): Json<SendOtp>,
) -> Result<(), AppError> {
    form.send_otp(state.db).await
}

pub async fn verify_otp(
    Extension(db): Extension<PgPool>,
    Json(form): Json<VerifyOtp>,
) -> Result<Json<Value>, AppError> {
    form.verify_otp(state.db).await
}

pub async fn update_password(
    Extension(db): Extension<PgPool>,
    Json(form): Json<UpdatePassword>,
) -> Result<Json<Value>, AppError> {
    form.update_password(state.db).await
}

pub fn router() -> Router {
    Router::new()
        .route("/forgot-password/send-otp", post(send_otp))
        .route("/forgot-password/verify-otp", post(verify_otp))
        .route("/forgot-password/update-password", patch(update_password))
}
