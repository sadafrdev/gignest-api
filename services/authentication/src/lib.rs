use axum::{
    Extension, Router,
    routing::{get, post},
};
use libs::authentication::forgot_password;
use libs::authentication::login;
use libs::authentication::register;
use libs::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/login", get(login::login))
        .route("/register", post(register::register))
        .route("/forgot_password/send_otp", get(forgot_password::send_otp))
        .route(
            "/forgot_password/verify_otp",
            get(forgot_password::verify_otp),
        )
        .route(
            "/forgot_password/update_password",
            get(forgot_password::update_password),
        )
        .layer(Extension(state))
}
