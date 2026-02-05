use axum::Extension;
use lib::AppState;

use axum::{
    Router,
    http::StatusCode,
    routing::{delete, get, post, patch},
    Json,
};
use crate::handlers::certificate::{Certificate, User, Users, certificates, generate_certificate, update_certificate, delete_certificate, CertificateID};

pub async fn create_certificate(
    Extension(state): Extension<AppState>,
    Json(payload): Json<Users>,
) -> Result<(), StatusCode> {
    generate_certificate(Extension(state), Json(payload)).await?;
    Ok(())
}

pub async fn get_certificates(
    Extension(state): Extension<AppState>,
    Json(payload): Json<User>,
) -> Result<Json<Vec<Certificate>>, StatusCode> {
    certificates(Extension(state), Json(payload)).await
}

pub async fn Update_certificate(
    Extension(state): Extension<AppState>,
    Json(payload): Json<Certificate>,
) -> Result<(), StatusCode> {
    update_certificate(Extension(state), Json(payload)).await?;
    Ok(())
}

pub async fn Delete_certificate(
    Extension(state): Extension<AppState>,
    Json(payload): Json<CertificateID>,
) -> Result<(), StatusCode> {
    delete_certificate(Extension(state), Json(payload)).await?;
    Ok(())
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/certificates", get(get_certificates))
        .route("/certificate", post(generate_certificate))
        .route("/update-certificate", patch(Update_certificate))
        .route("/delete-certificate", delete(Delete_certificate))
        .layer(Extension(state))
}
