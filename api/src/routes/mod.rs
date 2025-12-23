use crate::AppState;
use crate::authentication::login;
use crate::authentication::register;
use crate::authentication::forgot_password::{send_otp, verify_otp};
use axum::{Router, routing::{post,get}};

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .route("/login", get(login::login))
        .route("/register", post(register::register))
        .route("/send_otp", get(send_otp))
        .route("/verify_otp", get(verify_otp))
        .with_state(state)
}
