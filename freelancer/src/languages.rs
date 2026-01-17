use axum::{Extension, Json};
use axum::http::StatusCode;
use lib::AppState;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use axum::routing::post;
use lib::utils::enums::{Language, LanguageLevel};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UserLanguage {
    pub user_id: i32,
    pub language: Language,
    pub language_level: LanguageLevel,
}

pub async fn add_language(
    Extension(state): Extension<AppState>,
    Json(payload): Json<UserLanguage>,
) -> Result<StatusCode, StatusCode> {
    sqlx::query(
        r#"
        INSERT INTO languages (user_id, language, language_level)
        VALUES ($1, $2, $3)
        "#,
    )
    .bind(payload.user_id)
    .bind(payload.language as Language)
    .bind(payload.language_level as LanguageLevel)
    .execute(&state.db)
    .await
    .map_err(|e| {
        eprintln!("SQL ERROR: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(StatusCode::OK)
}

pub fn router(state: AppState) -> axum::Router {
    axum::Router::new()
        .route("/language", post(add_language))
        .layer(Extension(state))
}