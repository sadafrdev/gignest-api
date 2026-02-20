use crate::authentication;
use crate::clients;
use crate::freelancers;
use axum::{Extension, Router};
use utils::db::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .merge(freelancers::router(state.clone()))
        .merge(authentication::router(state.clone()))
        // .merge(clients::router(state.clone()))
        .layer(Extension(state))
}
