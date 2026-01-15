use axum::{
    Extension, Router,
    routing::{get, post},
};

use lib::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .layer(Extension(state))
}
