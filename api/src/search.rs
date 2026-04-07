use axum::{Extension, Json, Router, extract::Query, routing::get};
use utils::{db::DB, error::AppError};
use search::{
    clients::{Client, SearchClientParam},
    freelancers::{Freelancer, SearchFreelancerParam},
};

pub async fn search_client(
    Extension(db): Extension<DB>,
    Query(params): Query<SearchClientParam>,
) -> Result<Json<Vec<Client>>, AppError> {
    Client::search(&db, params).await.map(Json)
}

pub async fn search_freelancer(
    Extension(db): Extension<DB>,
    Query(params): Query<SearchFreelancerParam>,
) -> Result<Json<Vec<Freelancer>>, AppError> {
    Freelancer::search(&db, params).await.map(Json)
}

pub fn router() -> Router {
    Router::new()
        .route("/client/search", get(search_client))
        .route("/freelancer/search", get(search_freelancer))
}
