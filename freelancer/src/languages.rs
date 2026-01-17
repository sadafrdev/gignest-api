use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Extension, Json};
use lib::AppState;
use lib::utils::enums::{Language, LanguageLevel};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UserLanguage {
    pub user_id: i64,
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

#[derive(Deserialize, Serialize)]
pub struct UserId {
    user_id: i64,
}

pub async fn get_languages(
    Extension(state): Extension<AppState>,
    Json(payload): Json<UserId>,
) -> Result<Json<Vec<UserLanguage>>, StatusCode> {
    let languages = sqlx::query_as::<_, UserLanguage>(
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

    Ok(Json(languages))
}

pub fn router(state: AppState) -> axum::Router {
    axum::Router::new()
        .route("/language", post(add_language))
        .route("/languages", get(get_languages))
        .layer(Extension(state))
}
