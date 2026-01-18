use axum::{Extension, Router};
use lib::AppState;
use crate::jobs;

pub fn router(state: AppState) -> Router {
    Router::new()
        .layer(Extension(state))
}
