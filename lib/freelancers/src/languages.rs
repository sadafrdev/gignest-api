use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use utils::enums::{LanguageEnum, LanguageLevel};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Language {
    pub id: Option<i64>,
    pub user_id: Option<i64>,
    pub language: LanguageEnum,
    pub language_level: LanguageLevel,
}

impl Language {
    pub async fn add(db: PgPool, Json(payload): Json<Self>) -> Result<StatusCode, StatusCode> {
        sqlx::query(
            "
            INSERT INTO languages (id, user_id, language, language_level)
            VALUES ($1, $2, $3, $4)
            ",
        )
        .bind(payload.id)
        .bind(payload.user_id)
        .bind(payload.language as LanguageEnum)
        .bind(payload.language_level as LanguageLevel)
        .execute(&db)
        .await
        .map_err(|e| {
            eprintln!("SQL ERROR: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        Ok(StatusCode::OK)
    }

    pub async fn delete(db: PgPool, id: i64) -> Result<StatusCode, StatusCode> {
        sqlx::query(
            "
            DELETE FROM languages
            WHERE id = $1
            ",
        )
        .bind(id)
        .execute(&db)
        .await
        .map_err(|e| {
            eprintln!("SQL ERROR: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        Ok(StatusCode::OK)
    }

    pub async fn get(db: PgPool, id: i64) -> Result<Json<Vec<Language>>, StatusCode> {
        let languages = sqlx::query_as!(
            Language,
            r#"
                SELECT id, user_id, language AS "language: LanguageEnum", language_level AS "language_level: LanguageLevel"
                FROM languages
                WHERE user_id = $1
            "#,
            id
        )
        .fetch_all(&db)
        .await
        .map_err(|e| {
            eprintln!("SQL ERROR: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        Ok(Json(languages))
    }

    pub async fn update(db: PgPool, Json(payload): Json<Self>) -> Result<(), StatusCode> {
        sqlx::query(
            "
            UPDATE languages
            SET language = $2, language_level = $3
            WHERE id = $1
            ",
        )
        .bind(payload.id)
        .bind(payload.language as LanguageEnum)
        .bind(payload.language_level as LanguageLevel)
        .execute(&db)
        .await
        .map_err(|e| {
            eprintln!("SQL ERROR: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        Ok(())
    }
}
