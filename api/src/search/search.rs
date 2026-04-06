use axum::{Extension, Json};
use utils::{db::DB, error::AppError};
use search::search::{Client, SearchClientParam, Freelancer, SearchFreelancerParam};

pub async fn search_client(
    Extension(db): Extension<DB>,
    Json(params): Json<SearchClientParam>,
)  -> Result<Json<Vec<Client>>, AppError> {
    Client::search(&db, params).await.map(Json)
}

pub async fn search_freelancer(
    Extension(db): Extension<DB>,
    Json(params): Json<SearchFreelancerParam>,
) -> Result<Json<Vec<Freelancer>>, AppError> {
    Freelancer::search(&db, params).await.map(Json)
}