use axum::Extension;
use axum::{
    Json, Router,
    extract::Path,
    http::StatusCode,
    routing::{delete, get, patch, post},
};
use freelancers::certificates::Certificate;
use utils::db::AppState;

pub async fn create_certificate(
    Extension(state): Extension<AppState>,
    Json(payload): Json<Certificate>,
) -> Result<(), StatusCode> {
    Certificate::generate(Extension(state), Json(payload)).await?;
    Ok(())
}

pub async fn get_certificates(
    Extension(state): Extension<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Vec<Certificate>>, StatusCode> {
    Certificate::get(Extension(state), id).await
}

pub async fn update_certificate(
    Extension(state): Extension<AppState>,
    Json(payload): Json<Certificate>,
) -> Result<(), StatusCode> {
    Certificate::update(Extension(state), Json(payload)).await?;
    Ok(())
}

pub async fn delete_certificate(
    Extension(state): Extension<AppState>,
    Path(id): Path<i64>,
) -> Result<(), StatusCode> {
    Certificate::delete(Extension(state), id).await?;
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
