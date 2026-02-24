use authentication::forgot_password::{SendOtp, UpdatePassword, VerifyOtp};
use axum::Json;
use axum::Router;
use axum::routing::patch;
use axum::routing::post;
use axum::{extract::Extension, http::StatusCode};
use utils::db::AppState;

pub async fn send_otp(
    Extension(state): Extension<AppState>,
    Json(payload): Json<SendOtp>,
) -> Result<(), StatusCode> {
    SendOtp::send_otp(Extension(state), Json(payload)).await
}

pub async fn verify_otp(
    Extension(state): Extension<AppState>,
    Json(payload): Json<VerifyOtp>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    VerifyOtp::verify_otp(Extension(state), Json(payload)).await
}

pub async fn update_password(
    Extension(state): Extension<AppState>,
    Json(payload): Json<UpdatePassword>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    UpdatePassword::update_password(Extension(state), Json(payload)).await
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/forgot-password/send-otp", post(send_otp))
        .route("/forgot-password/verify-otp", post(verify_otp))
        .route("/forgot-password/update-password", patch(update_password))
        .layer(Extension(state))
}
