pub use crate::certificate;
pub use crate::languages;
use axum::{Extension, Router};
use lib::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .merge(certificate::router(state.clone()))
        .merge(languages::router(state.clone()))
        .layer(Extension(state))
}
