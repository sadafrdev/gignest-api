use axum::{Extension, Router};
use utils::db::DB;
use crate::authentication;
use crate::clients;
use crate::freelancers;

pub fn router(state: DB) -> Router {
    Router::new()
        .merge(authentication::router())
        .nest("/freelancer", freelancers::router())
        .nest("/client", clients::router())
        .layer(Extension(state))
}
