pub mod certificate;
pub mod languages;
pub mod routes;
use axum::Router;
use axum::routing::{delete, get, patch, post};
use axum::{Json, extract::Extension, http::StatusCode};
use languages::{Language, UpdateLanguage};
use lib::AppState;

pub async fn create_language(
    Extension(state): Extension<AppState>,
    Json(payload): Json<Language>,
) -> Result<(), StatusCode> {
    Language::add_language(Extension(state), Json(payload)).await?;
    Ok(())
}

pub async fn get_languages(
    Extension(state): Extension<AppState>,
    Json(payload): Json<languages::User>,
) -> Result<Json<Vec<Language>>, StatusCode> {
    let languages = Language::get_languages(Extension(state), Json(payload)).await?;
    Ok(Json(languages))
}

pub async fn update_language(
    Extension(state): Extension<AppState>,
    Json(payload): Json<UpdateLanguage>,
) -> Result<(), StatusCode> {
    Language::update_language(Extension(state), Json(payload)).await?;
    Ok(())
}

pub async fn delete_language(
    Extension(state): Extension<AppState>,
    Json(payload): Json<languages::LanguageID>,
) -> Result<(), StatusCode> {
    Language::delete_language(Extension(state), Json(payload)).await?;
    Ok(())
}

pub fn languages_router(state: AppState) -> Router {
    Router::new()
        .route("/language", post(create_language))
        .route("/delete-language", delete(delete_language))
        .route("/languages", get(get_languages))
        .route("/update-language", patch(update_language))
        .layer(Extension(state))
}
