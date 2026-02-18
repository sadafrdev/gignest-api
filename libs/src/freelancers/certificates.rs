use axum::http::StatusCode;
use axum::{Json, extract::Extension};
use crate::AppState;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDate;

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
pub struct Certificate {
    pub user_id: i64,
    pub name: String,
    pub certificate_by: String,
    pub year: NaiveDate,
}

impl Certificate {
    pub async fn generate(
        Extension(state): Extension<AppState>,
        Json(payload): Json<Self>,
    ) -> Result<(), StatusCode> {
        sqlx::query(
            "
            INSERT INTO certificates
            (user_id, name, certificate_by, year)
            VALUES ($1, $2, $3, $4)",
        )
        .bind(payload.user_id)
        .bind(payload.name)
        .bind(payload.certificate_by)
        .bind(payload.year)
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
        id: i64,
    ) -> Result<Json<Vec<Self>>, StatusCode> {
        let certificates = sqlx::query_as::<_, Self>(
            r#"
            SELECT
                user_id,
                name,
                certificate_by,
                year
            FROM certificates
            WHERE user_id = $1
            "#,
        )
        .bind(id)
        .fetch_all(&state.db)
        .await
        .map_err(|e| {
            eprintln!("SQL ERROR: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        Ok(Json(certificates))
    }

    pub async fn update(
        Extension(state): Extension<AppState>,
        Json(payload): Json<Self>,
    ) -> Result<StatusCode, StatusCode> {
        let result = sqlx::query(
            r#"
                UPDATE certificates
                SET
                    name = $1,
                    certificate_by = $2,
                    year = $3
                WHERE user_id = $4
            "#,
        )
        .bind(&payload.name)
        .bind(&payload.certificate_by)
        .bind(payload.year)
        .bind(payload.user_id)
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

    pub async fn delete(
        Extension(state): Extension<AppState>,
        id: i64,
    ) -> Result<StatusCode, StatusCode> {
        let result = sqlx::query(
            "
                DELETE FROM certificates
                WHERE id = $1
            ",
        )
        .bind(id)
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
