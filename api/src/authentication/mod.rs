use axum::{
    Extension, Router,
    routing::{get, post},
};
use lib_authentication::forgot_password;
use lib_authentication::login;
use lib_authentication::register;
use utils::db::AppState;

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
