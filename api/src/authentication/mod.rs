use axum::{
    Extension, Router,
    routing::{get, post},
};
pub mod login;
pub mod forgot_password;
pub mod register;
use utils::db::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .merge(login::router(state.clone()))
        .merge(register::router(state.clone()))
        .merge(forgot_password::router(state.clone()))
        .layer(Extension(state))
}
