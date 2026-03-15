use crate::authentication;
use crate::clients;
use crate::freelancers;
use axum::{Extension, Router};
use utils::db::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .merge(freelancers::router())
        .merge(authentication::router())
        .merge(clients::router())
        .layer(Extension(state))
}
