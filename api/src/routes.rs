use axum::{Extension, Router};
use utils::db::AppState;
use crate::authentication;
use crate::clients;
use crate::freelancers;

pub fn router(state: AppState) -> Router {
    Router::new()
        .merge(freelancers::router())
        .merge(authentication::router())
        .merge(clients::router())
        .layer(Extension(state))
}
