use crate::job_router;
use axum::{Extension, Router};
use lib::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .merge(job_router(state.clone()))
        .layer(Extension(state))
}
