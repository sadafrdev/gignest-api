use axum::{
    Json, Router,
    extract::Extension,
    http::StatusCode,
    routing::{patch, post},
};
use sqlx::PgPool;
use authentication::forgot_password::{SendOtp, UpdatePassword, VerifyOtp};

pub async fn send_otp(
    Extension(db): Extension<PgPool>,
    Json(form): Json<SendOtp>,
) -> Result<(), StatusCode> {
    form.send_otp(db).await
}

pub async fn verify_otp(
    Extension(db): Extension<PgPool>,
    Json(form): Json<VerifyOtp>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    form.verify_otp(db).await
}

pub async fn update_password(
    Extension(db): Extension<PgPool>,
    Json(form): Json<UpdatePassword>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    form.update_password(db).await
}

pub fn router() -> Router {
    Router::new()
        .route("/forgot-password/send-otp", post(send_otp))
        .route("/forgot-password/verify-otp", post(verify_otp))
        .route("/forgot-password/update-password", patch(update_password))
}
