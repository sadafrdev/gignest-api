use axum::http::StatusCode;
use axum::{Json, extract::Extension};
use axum::{
    Router,
    routing::{get, patch, post},
};
use lib::AppState;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDate;
use lib::utils::enums::Country;

#[derive(Deserialize, Serialize)]
pub struct Users {
    pub user_id: i64,
    pub country: Country,
    pub degree: String,
    pub institute: String,
    pub major: String,
    pub year_of_graduation: NaiveDate,
}

pub async fn create_education(
    Extension(state): Extension<AppState>,
    Json(payload): Json<Users>,
) -> Result<(), StatusCode> {
    sqlx::query(
        "
        INSERT INTO educations
        (user_id, country, degree, institute, major, year_of_graduation)
        VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(payload.user_id)
    .bind(payload.country as Country)
    .bind(payload.degree)
    .bind(payload.institute)
    .bind(payload.major)
    .bind(payload.year_of_graduation)
    .execute(&state.db)
    .await
    .map_err(|e| {
        eprintln!("SQL ERROR: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(())
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/education", post(create_education))
        .layer(Extension(state))
}
