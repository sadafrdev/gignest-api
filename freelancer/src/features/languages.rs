use crate::handlers::languages;
use axum::Extension;
use axum::{
    Json, Router,
    http::StatusCode,
    routing::{delete, get, patch, post},
};
use languages::Language;
use lib::AppState;

pub async fn create_language(
    Extension(state): Extension<AppState>,
    Json(payload): Json<Language>,
) -> Result<(), StatusCode> {
    Language::add(Extension(state), Json(payload)).await?;
    Ok(())
}

pub async fn get_languages(
    Extension(state): Extension<AppState>,
    Json(payload): Json<languages::User>,
) -> Result<Json<Vec<Language>>, StatusCode> {
    let languages = Language::get(Extension(state), Json(payload)).await?;
    Ok(Json(languages))
}

pub async fn update_language(
    Extension(state): Extension<AppState>,
    Json(payload): Json<Language>,
) -> Result<(), StatusCode> {
    Language::update(Extension(state), Json(payload)).await?;
    Ok(())
}

pub async fn delete_language(
    Extension(state): Extension<AppState>,
    Json(payload): Json<languages::LanguageID>,
) -> Result<(), StatusCode> {
    Language::delete(Extension(state), Json(payload)).await?;
    Ok(())
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/language", post(create_language))
        .route("/delete-language", delete(delete_language))
        .route("/languages", get(get_languages))
        .route("/update-language", patch(update_language))
        .layer(Extension(state))
}
