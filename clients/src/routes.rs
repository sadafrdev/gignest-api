use crate::jobs;
use axum::{Extension, Router};
use lib::AppState;
use crate::job_router;

pub fn router(state: AppState) -> Router {
    Router::new()
        .merge(job_router(state.clone()))
        .layer(Extension(state))
}
