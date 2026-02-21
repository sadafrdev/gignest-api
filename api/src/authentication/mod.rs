use axum::{
    Extension, Router,
    routing::{get, post},
};
pub mod login;
pub mod register;
use authentication::forgot_password;
use utils::db::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .merge(login::router(state.clone()))
        .merge(register::router(state.clone()))
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
