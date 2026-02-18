use axum::{Extension, Router};
use libs::AppState;
pub mod jobs;

pub fn router(state: AppState) -> Router {
    Router::new()
        .merge(jobs::router(state.clone()))
        .layer(Extension(state))
}
