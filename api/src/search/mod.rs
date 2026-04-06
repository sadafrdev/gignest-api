use axum::Router;
use axum::routing::get;
use utils::middleware::from_func;
pub mod search;

pub fn router() -> Router {
    Router::new()
        .route("/client/search", get(search::search_client))
        .route("/freelancer/search", get(search::search_freelancer))
}