use axum::middleware;
use axum::{Extension, Router};
use utils::db::DB;
use utils::middleware::{verify_role, verify_token};
use crate::authentication;
use crate::clients;
use crate::freelancers;

pub fn router(state: DB) -> Router {
    let routes = Router::new()
        .nest("/freelancer", freelancers::router())
        .nest("/client", clients::router())
        .layer(middleware::from_fn(verify_role))
        .layer(middleware::from_fn(verify_token));

        Router::new()
            .merge(authentication::router())
            .merge(routes)
            .layer(Extension(state))
}
