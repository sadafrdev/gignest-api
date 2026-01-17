use core::str;

use axum::http::{StatusCode, response};
use axum::{Json, extract::Extension};
use axum::{
    Router,
    routing::{get, patch, post},
};
use lib::AppState;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDate;
use lib::utils::enums::Country;

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
pub struct Education {
    pub user_id: i64,
    pub country: Country,
    pub degree: String,
    pub institute: String,
    pub major: String,
    pub year_of_graduation: NaiveDate,
}

pub async fn create_education(
    Extension(state): Extension<AppState>,
    Json(payload): Json<Education>,
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

#[derive(serde::Deserialize)]
pub struct User {
    user_id: i64,
}

pub async fn get_educations(
    Extension(state): Extension<AppState>,
    Json(payload): Json<User>,
) -> Result<Json<Vec<Education>>, StatusCode> {
    let educations = sqlx::query_as::<_, Education>(
        r#"
            SELECT
                user_id,
                country,
                degree,
                institute,
                major,
                year_of_graduation
            FROM educations
            WHERE user_id = $1
        "#,
    )
    .bind(payload.user_id)
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        eprintln!("SQL ERROR: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
   
    Ok(Json(educations))
}

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
pub struct UpdateEducation {
    pub id: i64,
    pub country: Country,
    pub degree: String,
    pub institute: String,
    pub major: String,
    pub year_of_graduation: NaiveDate,
}

pub async fn update_education(
    Extension(state): Extension<AppState>,
    Json(payload): Json<UpdateEducation>,
) -> Result<(), StatusCode> {
    sqlx::query(
        "
            UPDATE educations
            SET country = $2, degree = $3, institute = $4, major = $5, year_of_graduation = $6
            WHERE id = $1
        ",
    )
    .bind(payload.id)
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
        .route("/educations", get(get_educations))
        .route("/update-education", patch(update_education))
        .layer(Extension(state))
}

