pub use crate::certificate;
use axum::{Extension, Router};
use lib::AppState;
pub use crate::education;

pub fn router(state: AppState) -> Router {
    Router::new()
        .merge(certificate::router(state.clone()))
        .merge(education::router(state.clone()))
        .layer(Extension(state))
}
