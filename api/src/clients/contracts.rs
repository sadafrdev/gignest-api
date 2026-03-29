use axum::{Json, routing::post, extract::Extension, Router};
use utils::{db::DB, error::AppError};
use clients::contracts::{AcceptProposal};

pub async fn accept_proposal(
    Extension(db): Extension<DB>,
    Json(form): Json<AcceptProposal>,
) -> Result<(), AppError> {
    form.accept_proposal_and_create_contract(db).await
}

pub fn router() -> Router {
    Router::new()
        .route("/accept-proposal", post(accept_proposal))
}
