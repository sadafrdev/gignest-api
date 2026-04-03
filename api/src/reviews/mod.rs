use axum::Router;
use axum::middleware;
use utils::middleware::from_func;
pub mod create;
pub mod find;

pub fn router() -> Router {
    let reviews_router = Router::new()
        .merge(create::router())
        .merge(find::router())
        .layer(middleware::from_fn(from_func));

    Router::new().nest("/reviews", reviews_router)
}
