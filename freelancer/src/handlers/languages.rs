use axum::http::StatusCode;
use axum::{Extension, Json};
use lib::AppState;
use lib::utils::enums::{LanguageEnum, LanguageLevel};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Language {
    pub user_id: i64,
    pub language: LanguageEnum,
    pub language_level: LanguageLevel,
}

#[derive(Deserialize, Serialize)]
pub struct User {
    user_id: i64,
}

#[derive(Deserialize, Serialize, sqlx::FromRow, Debug)]
pub struct UpdateLanguage {
    pub id: i64,
    pub language: LanguageEnum,
    pub language_level: LanguageLevel,
}

#[derive(Deserialize, Serialize)]
pub struct LanguageID {
    pub id: i64,
}

impl Language {
    pub async fn add_language(
        Extension(state): Extension<AppState>,
        Json(payload): Json<Language>,
    ) -> Result<StatusCode, StatusCode> {
        sqlx::query(
            r#"
            INSERT INTO languages (user_id, language, language_level)
            VALUES ($1, $2, $3)
            "#,
        )
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

    pub async fn delete_language(
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

    pub async fn get_languages(
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

    pub async fn update_language(
        Extension(state): Extension<AppState>,
        Json(payload): Json<UpdateLanguage>,
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
