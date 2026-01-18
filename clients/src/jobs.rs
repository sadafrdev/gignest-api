use axum::http::StatusCode;
use axum::{Extension, Json};
use axum::{
    Router,
    routing::{get, post},
};
use bigdecimal::BigDecimal;
use core::str;
use lib::AppState;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
pub struct Job {
    pub client_id: i64,
    pub title: String,
    pub description: String,
    pub budget_min: BigDecimal,
    pub budget_max: BigDecimal,
}

pub async fn create_job(
    Extension(state): Extension<AppState>,
    Json(payload): Json<Job>,
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

#[derive(Deserialize, Serialize, Debug)]
pub struct ClientID {
    pub client_id: i64,
}

pub async fn get_jobs(
    Extension(state): Extension<AppState>,
    Json(payload): Json<ClientID>,
) -> Result<Json<Option<Job>>, StatusCode> {
    let jobs = sqlx::query_as::<_, Job>(
        r#"
        SELECT
            client_id,
            title,
            description,
            budget_min,
            budget_max
        FROM jobs
        WHERE client_id = $1
        "#,
    )
    .bind(payload.client_id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        eprintln!("SQL ERROR: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(jobs))
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/job", post(create_job))
        .route("/jobs", get(get_jobs))
        .layer(Extension(state))
}
