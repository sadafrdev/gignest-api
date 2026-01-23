pub mod handlers;
pub mod routes;
use axum::{
    Extension, Router,
    routing::{get, post},
};
use core::authentication::{login, register::Users, forgot_password::{send_otp, verify_otp, update_password}};
use core::AppState;

pub async fn login(
    Extension(state): Extension<AppState>,
    Json(form): Json<login::Login>,
) -> Result<(), StatusCode> {
    form.login(state.db).await
}

pub async fn register(
    Extension(state): Extension<AppState>,
    Json(form): Json<Users>,
) -> Result<(), StatusCode> {
    form.register(state.db).await
}

pub async fn send_otp(
    Extension(state): Extension<AppState>,
    Json(form): Json<send_otp>,
) -> Result<(), StatusCode> {
    form.send_otp(&state.db).await
}

pub async fn verify_token(
    Extension(state): Extension<AppState>,
    Json(form): Json<verify_otp>,
){
    form.verify_otp(state.db).await
}

pub async fn update_password(
    Extension(state): Extension<AppState>,
    Json(form): Json<verify_otp>,
){
    form.update_password(state.db).await
}

pub fn router() -> Router {
    Router::new()
        .route("/login", get(login))
        .route("/register", post(register))
        .route("/forgot_password/send_otp", get(send_otp))
        .route(
            "/forgot_password/verify_otp",
            get(verify_otp),
        )
        .route(
            "/forgot_password/update_password",
            get(update_password),
        )
}