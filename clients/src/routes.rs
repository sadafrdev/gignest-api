use crate::jobs;
use axum::{Extension, Router};
use lib::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .merge(jobs::router(state.clone()))
        .layer(Extension(state))
}
