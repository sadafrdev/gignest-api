use axum::Router;
pub mod forgot_password;
pub mod login;
pub mod register;

pub fn router() -> Router {
    Router::new()
        .merge(login::router())
        .merge(register::router())
        .merge(forgot_password::router())

}
