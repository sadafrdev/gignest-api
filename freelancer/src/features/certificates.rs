use crate::handlers::certificate::{Certificate, CertificateID, User};
use axum::Extension;
use axum::{
    Json, Router,
    http::StatusCode,
    routing::{delete, get, patch, post},
};
use lib::AppState;

pub async fn create_certificate(
    Extension(state): Extension<AppState>,
    Json(payload): Json<Certificate>,
) -> Result<(), StatusCode> {
    Certificate::generate(Extension(state), Json(payload)).await?;
    Ok(())
}

pub async fn get_certificates(
    Extension(state): Extension<AppState>,
    Json(payload): Json<User>,
) -> Result<Json<Vec<Certificate>>, StatusCode> {
    Certificate::get(Extension(state), Json(payload)).await
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
    Json(payload): Json<CertificateID>,
) -> Result<(), StatusCode> {
    Certificate::delete(Extension(state), Json(payload)).await?;
    Ok(())
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/certificates", get(get_certificates))
        .route("/certificate", post(create_certificate))
        .route("/update-certificate", patch(update_certificate))
        .route("/delete-certificate", delete(delete_certificate))
        .layer(Extension(state))
}
