use axum::http::StatusCode;
use axum::{Json, extract::Extension};
use core::str;
use lib::AppState;
use lib::utils::enums::Country;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDate;

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
pub struct Education {
    pub user_id: i64,
    pub country: Country,
    pub degree: String,
    pub institute: String,
    pub major: String,
    pub year_of_graduation: NaiveDate,
}

#[derive(serde::Deserialize)]
pub struct User {
    user_id: i64,
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

#[derive(Deserialize, Serialize, Debug)]
pub struct DeleteEducation {
    pub id: i64,
}

impl Education {
    pub async fn create(
        Extension(state): Extension<AppState>,
        Json(payload): Json<Self>,
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

    pub async fn get(
        Extension(state): Extension<AppState>,
        Json(payload): Json<User>,
    ) -> Result<Json<Vec<Self>>, StatusCode> {
        let educations = sqlx::query_as::<_, Self>(
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

    pub async fn update(
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

    pub async fn delete(
        Extension(state): Extension<AppState>,
        Json(payload): Json<DeleteEducation>,
    ) -> Result<StatusCode, StatusCode> {
        let result = sqlx::query(
            "
                DELETE FROM educations
                WHERE id = $1
            ",
        )
        .bind(payload.id)
        .execute(&state.db)
        .await
        .map_err(|e| {
            eprintln!("SQL ERROR: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        if result.rows_affected() == 0 {
            return Err(StatusCode::NOT_FOUND);
        }

        Ok(StatusCode::OK)
    }
}
