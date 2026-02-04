use axum::Extension;
use lib::AppState;
pub mod certificate;
pub mod languages;
pub mod educations;
pub mod routes;
pub mod skills;
use axum::{
    Router,
    http::StatusCode,
    routing::{delete, get, post, put, patch},
    Json,
    extract::Path,
};
use skills::{Skills, UpdateSkill};
use languages::{Language, UpdateLanguage};

pub async fn add_skill(
    Extension(state): Extension<AppState>,
    Json(payload): Json<Skills>,
) -> Result<(), StatusCode> {
    Skills::generate_skill(Extension(state), Json(payload)).await
}

pub async fn get_skills(
    Extension(state): Extension<AppState>,
    Path(user_id): Path<i64>,
) -> Result<Json<Vec<Skills>>, StatusCode> {
    Skills::get_skills(Extension(state), user_id).await
}

pub async fn update_skill(
    Extension(state): Extension<AppState>,
    Json(payload): Json<UpdateSkill>,
) -> Result<(), StatusCode> {
    Skills::update_skill(Extension(state), Json(payload)).await
}

pub async fn delete_skill(
    Extension(state): Extension<AppState>,
    Path(id): Path<i64>,
) -> Result<(), StatusCode> {
    Skills::delete_skill(Extension(state), id).await
}

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

pub fn skills_routes(state: AppState) -> Router {
    Router::new()
        .route("/skill", post(add_skill))
        .route("/skills/{id}", get(get_skills))
        .route("/update-skill", put(update_skill))
        .route("/delete-skill/{id}", delete(delete_skill))
        .layer(Extension(state))
}