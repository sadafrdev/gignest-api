use axum::{Extension, Router};
pub mod forgot_password;
pub mod login;
pub mod register;
use utils::db::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .merge(login::router(state.clone()))
        .merge(register::router(state.clone()))
        .merge(forgot_password::router(state.clone()))
        .layer(Extension(state))
}
