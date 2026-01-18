pub use crate::certificate;
use crate::skills;
use axum::{Extension, Router};
use lib::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .merge(certificate::router(state.clone()))
        .merge(skills::router(state.clone()))
        .layer(Extension(state))
}
