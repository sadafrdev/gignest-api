use crate::authentication;
use crate::clients;
use crate::freelancers;
use axum::{Extension, Router};
use utils::db::AppState;
use axum::{middleware};
use utils::middleware::from_func;

pub fn router(state: AppState) -> Router {
    Router::new()
        .merge(freelancers::router(state.clone()))
        .merge(authentication::router(state.clone()))
        .merge(clients::router(state.clone()))
        .layer(Extension(state))
        .layer(middleware::from_fn(from_func))
}
