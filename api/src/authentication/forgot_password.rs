use authentication::forgot_password::{SendOtp, UpdatePassword, VerifyOtp};
use axum::Json;
use axum::Router;
use axum::routing::patch;
use axum::routing::post;
use axum::{extract::Extension, http::StatusCode};
use utils::db::AppState;

pub async fn send_otp(
    Extension(state): Extension<AppState>,
    Json(form): Json<SendOtp>,
) -> Result<(), StatusCode> {
    form.send_otp(state.db).await
}

pub async fn verify_otp(
    Extension(state): Extension<AppState>,
    Json(form): Json<VerifyOtp>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    form.verify_otp(state.db).await
}

pub async fn update_password(
    Extension(state): Extension<AppState>,
    Json(form): Json<UpdatePassword>,
) -> Result<Json<serde_json::Value>, StatusCode> {
   form.update_password(state.db).await
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/forgot-password/send-otp", post(send_otp))
        .route("/forgot-password/verify-otp", post(verify_otp))
        .route("/forgot-password/update-password", patch(update_password))
        .layer(Extension(state))
}
