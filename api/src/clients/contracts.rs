use axum::{Json, Router, extract::{Extension, Path}, routing::{post, get}};
use utils::{db::DB, error::AppError};
use clients::contracts::{Contract};
use clients::contracts;

pub async fn accept_proposal(
    Extension(db): Extension<DB>,
    Json(form): Json<Contract>,
) -> Result<(), AppError> {
    form.accept_proposal_and_create_contract(db).await
}

pub async fn client_contracts(
    Path(id): Path<i64>,
    Extension(db): Extension<DB>,
) -> Result<Json<Vec<Contract>>, AppError> {
    contracts::client_contracts(id, db).await
}

pub async fn freelancer_contracts(
    Path(id): Path<i64>,
    Extension(db): Extension<DB>,
) -> Result<Json<Vec<Contract>>, AppError> {
    contracts::freelancer_contracts(id, db).await
}

pub async fn complete_contract(
    Path(id): Path<i64>,
    Extension(db): Extension<DB>
) -> Result<(), AppError> {
    contracts::complete_contract(id, db).await
}

pub fn router() -> Router {
    Router::new()
        .route("/accept-proposal", post(accept_proposal))
        .route("/get-client-contracts/{id}", get(client_contracts))
        .route("/get-freelancer-contracts/{id}", get(freelancer_contracts))
        .route("/complete-contract/{id}", post(complete_contract))
}