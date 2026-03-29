use axum::{Json, Router, extract::{Extension, Path}, routing::{delete, post}};
use utils::{db::DB, error::AppError};
use clients::contracts::{AcceptProposal};
use clients::contracts;

pub async fn accept_proposal(
    Extension(db): Extension<DB>,
    Json(form): Json<AcceptProposal>,
) -> Result<(), AppError> {
    form.accept_proposal_and_create_contract(db).await
}

pub async fn delete_contract(
    Path(id): Path<i64>,
    Extension(db): Extension<DB>,
) -> Result<(), AppError> {
    contracts::delete_contract(id, db).await
}

pub fn router() -> Router {
    Router::new()
        .route("/accept-proposal", post(accept_proposal))
        .route("/delete-contract/{id}", delete(delete_contract))
}