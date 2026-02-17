use axum::http::StatusCode;
use axum::{Extension, Json};
use lib::AppState;
use lib::utils::enums::{LanguageEnum, LanguageLevel};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Language {
    pub id: i64,
    pub user_id: i64,
    pub language: LanguageEnum,
    pub language_level: LanguageLevel,
}

#[derive(Deserialize, Serialize)]
pub struct User {
    user_id: i64,
}

#[derive(Deserialize, Serialize)]
pub struct LanguageID {
    pub id: i64,
}

impl Language {
    pub async fn add(
        Extension(state): Extension<AppState>,
        Json(payload): Json<Self>,
    ) -> Result<StatusCode, StatusCode> {
        sqlx::query(
            r#"
            INSERT INTO languages (id, user_id, language, language_level)
            VALUES ($1, $2, $3, $4)
            "#,
        )
        .bind(payload.id)
        .bind(payload.user_id)
        .bind(payload.language as LanguageEnum)
        .bind(payload.language_level as LanguageLevel)
        .execute(&state.db)
        .await
        .map_err(|e| {
            eprintln!("SQL ERROR: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        Ok(StatusCode::OK)
    }

    pub async fn delete(
        Extension(state): Extension<AppState>,
        Json(payload): Json<LanguageID>,
    ) -> Result<StatusCode, StatusCode> {
        sqlx::query(
            r#"
            DELETE FROM languages
            WHERE id = $1
            "#,
        )
        .bind(payload.id)
        .execute(&state.db)
        .await
        .map_err(|e| {
            eprintln!("SQL ERROR: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        Ok(StatusCode::OK)
    }

    pub async fn get(
        Extension(state): Extension<AppState>,
        Json(payload): Json<User>,
    ) -> Result<Vec<Self>, StatusCode> {
        let languages = sqlx::query_as::<_, Self>(
            r#"
            SELECT user_id, language, language_level
            FROM languages
            WHERE user_id = $1
            "#,
        )
        .bind(payload.user_id)
        .fetch_all(&state.db)
        .await
        .map_err(|e| {
            eprintln!("SQL ERROR: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        Ok(languages)
    }

    pub async fn update(
        Extension(state): Extension<AppState>,
        Json(payload): Json<Self>,
    ) -> Result<(), StatusCode> {
        sqlx::query(
            r#"
            UPDATE languages
            SET language = $2, language_level = $3
            WHERE id = $1
            "#,
        )
        .bind(payload.id)
        .bind(payload.language as LanguageEnum)
        .bind(payload.language_level as LanguageLevel)
        .execute(&state.db)
        .await
        .map_err(|e| {
            eprintln!("SQL ERROR: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        Ok(())
    }
}
