use axum::{
    Extension, Json, Router,
    extract::Path,
    routing::{delete, get, patch, post},
};
use utils::{db::DB, error::AppError};
use freelancers::certificates::{Certificate, UpdateCertificate, User, CertificateDelete};

pub async fn create_certificate(
    Extension(db): Extension<DB>,
    Json(form): Json<Certificate>,
) -> Result<(), AppError> {
    form.generate(db).await
}

pub async fn get_certificates(
    Path(id): Path<User>,
    Extension(db): Extension<DB>,
) -> Result<Json<Vec<Certificate>>, AppError> {
    id.get(db).await
}

pub async fn update_certificate(
    Extension(db): Extension<DB>,
    Json(form): Json<UpdateCertificate>,
) -> Result<(), AppError> {
    form.update(db).await
}

pub async fn delete_certificate(
    Path(id): Path<CertificateDelete>,
    Extension(db): Extension<DB>,
) -> Result<(), AppError> {
    id.delete(db).await
}

pub fn router() -> Router {
    Router::new()
        .route("/certificates/{id}", get(get_certificates))
        .route("/certificate", post(create_certificate))
        .route("/update-certificate", patch(update_certificate))
        .route("/delete-certificate/{id}", delete(delete_certificate))
}
