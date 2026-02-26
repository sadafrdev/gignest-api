use axum::Extension;
use axum::{
    Json, Router,
    extract::Path,
    routing::{delete, get, patch, post},
};
use freelancers::certificates::Certificate;
use utils::{db::AppState, error::AppError};

pub async fn create_certificate(
    Extension(state): Extension<AppState>,
    Json(form): Json<Certificate>,
) -> Result<(), AppError> {
    form.generate(state.db).await?;
    Ok(())
}

pub async fn get_certificates(
    Extension(state): Extension<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Vec<Certificate>>, AppError> {
    Certificate.get(state.db).await
}

pub async fn update_certificate(
    Extension(state): Extension<AppState>,
    Json(form): Json<Certificate>,
) -> Result<(), AppError> {
    form.update(state.db).await?;
    Ok(())
}

pub async fn delete_certificate(
    Extension(state): Extension<AppState>,
    Path(id): Path<i64>,
) -> Result<(), AppError> {
    form.delete(state.db).await?;
    Ok(())
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/certificates/{id}", get(get_certificates))
        .route("/certificate", post(create_certificate))
        .route("/update-certificate", patch(update_certificate))
        .route("/delete-certificate/{id}", delete(delete_certificate))
        .layer(Extension(state))
}
