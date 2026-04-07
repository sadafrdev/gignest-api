use axum::{
    Json, Router,
    extract::{Extension, Path},
    routing::{get, post},
};
use utils::{db::DB, error::AppError};
use clients::contracts::{self, Contract};

pub async fn accept_proposal(
    Extension(db): Extension<DB>,
    Json(form): Json<Contract>,
) -> Result<(), AppError> {
    form.accept_proposal_and_create_contract(db).await
}

pub async fn client_contracts_by_id(
    Path(id): Path<i64>,
    Extension(db): Extension<DB>,
) -> Result<Json<Vec<Contract>>, AppError> {
    Contract::client_contracts_by_id(id, db).await.map(Json)
}

pub async fn freelancer_contracts_by_id(
    Path(id): Path<i64>,
    Extension(db): Extension<DB>,
) -> Result<Json<Vec<Contract>>, AppError> {
    Contract::freelancer_contracts_by_id(id, db).await.map(Json)
}

pub async fn complete_contract(
    Path(id): Path<i64>,
    Extension(db): Extension<DB>,
) -> Result<(), AppError> {
    contracts::complete_contract(id, db).await
}

pub fn router() -> Router {
    Router::new()
        .route("/proposal/accept", post(accept_proposal))
        .route("/client/contracts/{id}", get(client_contracts_by_id))
        .route(
            "/freelancer/contracts/{id}",
            get(freelancer_contracts_by_id),
        )
        .route("/contract/complete/{id}", post(complete_contract))
}
