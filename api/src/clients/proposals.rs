use axum::Json;
use axum::routing::post;
use axum::extract::Extension;
use clients::proposals::Proposal;
use utils::{error::AppError, db::AppState};

pub async fn generate(
    Extension(state): Extension<AppState>,
    Json(form): Json<Proposal>,
) -> Result<(), AppError> {
    form.create(state.db).await
}

pub fn router(state: AppState) -> axum::Router {
    axum::Router::new()
        .route("/proposal", post(generate))
        .layer(Extension(state))
}
