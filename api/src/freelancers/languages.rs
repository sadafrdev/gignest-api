use axum::Extension;
use axum::{
    Json, Router,
    extract::Path,
    routing::{delete, get, patch, post},
};
pub use freelancers::languages::Language;
use utils::{db::AppState, error::AppError};

pub async fn create_language(
    Extension(state): Extension<AppState>,
    Json(form): Json<Language>,
) -> Result<(), AppError> {
    form.add(state.db).await?;
    Ok(())
}

pub async fn get_languages(
    Extension(state): Extension<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Vec<Language>>, AppError> {
    let languages = Language::get(state.db).await?;
    Ok(Json(languages))
}

pub async fn update_language(
    Extension(state): Extension<AppState>,
    Json(form): Json<Language>,
) -> Result<(), AppError> {
    form.update(state.db).await?;
    Ok(())
}

pub async fn delete_language(
    Extension(state): Extension<AppState>,
    Path(id): Path<i64>,
) -> Result<(), AppError> {
    Language::delete(state.db).await?;
    Ok(())
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/language", post(create_language))
        .route("/delete-language/{id}", delete(delete_language))
        .route("/languages", get(get_languages))
        .route("/update-language/{id}", patch(update_language))
        .layer(Extension(state))
}
