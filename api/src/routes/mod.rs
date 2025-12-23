use crate::AppState;
use crate::authentication::login;
use crate::authentication::register;
use crate::authentication::forgot_password::{send_otp, verify_otp, update_password};
use axum::{Router, routing::{post,get}};

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .route("/login", get(login::login))
        .route("/register", post(register::register))
        .route("/forgot_password/send_otp", get(send_otp))
        .route("/orgot_password/verify_otp", get(verify_otp))
        .route("/orgot_password/update_password", get(update_password))
        .with_state(state)
}
