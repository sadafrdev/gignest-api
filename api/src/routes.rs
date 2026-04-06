use axum::{Extension, Router, middleware};
use utils::db::DB;
use utils::middleware::{from_func};
use crate::{authentication, search, clients, freelancers};

pub fn router(state: DB) -> Router {
    let routes = Router::new()
        .nest("/freelancer", freelancers::router())
        .nest("/client", clients::router())
        .layer(middleware::from_fn(from_func));

    Router::new()
        .merge(authentication::router())
        .merge(routes)
        .nest("/search", search::router())
        .layer(Extension(state))
}
