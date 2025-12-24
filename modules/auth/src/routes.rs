use axum::{
    Router,
    routing::{get, post},
};

pub fn router() -> Router {
    Router::new()
        .route("/login", get(crate::handlers::login))
        .route("/register", post(crate::handlers::register))
        .route("/forgot_password/send_otp", get(crate::handlers::send_otp))
        .route(
            "/forgot_password/verify_otp",
            get(crate::handlers::verify_otp),
        )
        .route(
            "/forgot_password/update_password",
            get(crate::handlers::update_password),
        )
}
