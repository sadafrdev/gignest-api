use axum::{Extension, Router};
use utils::db::DB;
use crate::authentication;
use crate::clients;
use crate::freelancers;

pub fn router(state: DB) -> Router {
    Router::new()
        .merge(freelancers::router())
        .merge(authentication::router())
        .merge(clients::router())
        .layer(Extension(state))
}
