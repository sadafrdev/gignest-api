use axum::{Extension, Json};
use axum::http::StatusCode;
use lib::AppState;
use serde::{Deserialize, Serialize};
use axum::{
    Router,
    routing::post,
};

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateJob {
    pub client_id: i64,
    pub title: String,
    pub description: String,
    pub budget_min: f64,
    pub budget_max: f64,
}

pub async fn create_job(
    Extension(state): Extension<AppState>,
    Json(payload): Json<CreateJob>,
) -> Result<(), StatusCode> {
    sqlx::query(
        "
        INSERT INTO jobs (client_id, title, description, budget_min, budget_max)
        VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(payload.client_id)
    .bind(&payload.title)
    .bind(&payload.description)
    .bind(payload.budget_min)
    .bind(payload.budget_max)
    .execute(&state.db)
    .await
    .map_err(|e| {
        eprintln!("SQL ERROR: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(())
}
pub async fn router(state: AppState) -> Router {
    Router::new()
        .route("/jobs", post(create_job))
        .layer(Extension(state))
}