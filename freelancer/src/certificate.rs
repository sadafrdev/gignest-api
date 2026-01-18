use axum::http::StatusCode;
use axum::{Json, extract::Extension};
use axum::{
    Router,
    routing::{delete, get, patch, post},
};
use lib::AppState;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDate;

#[derive(Deserialize, Serialize)]
pub struct Users {
    pub user_id: i64,
    pub name: String,
    pub certificate_by: String,
    pub year: NaiveDate,
}

pub async fn generate_certificate(
    Extension(state): Extension<AppState>,
    Json(payload): Json<Users>,
) -> Result<(), StatusCode> {
    sqlx::query(
        "
        INSERT INTO certificates
        (user_id, name, certificate_by, year)
        VALUES ($1, $2, $3, $4)",
    )
    .bind(payload.user_id)
    .bind(payload.name)
    .bind(payload.certificate_by)
    .bind(payload.year)
    .execute(&state.db)
    .await
    .map_err(|e| {
        eprintln!("SQL ERROR: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(())
}

#[derive(serde::Deserialize)]
pub struct User {
    user_id: i64,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Certificate {
    pub user_id: i64,
    pub name: String,
    pub certificate_by: String,
    pub year: NaiveDate,
}

pub async fn get_certificates(
    Extension(state): Extension<AppState>,
    Json(payload): Json<User>,
) -> Result<Json<Vec<Certificate>>, StatusCode> {
    let certificates = sqlx::query_as::<_, Certificate>(
        r#"
        SELECT
            user_id,
            name,
            certificate_by,
            year
        FROM certificates
        WHERE user_id = $1
        "#,
    )
    .bind(payload.user_id)
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        eprintln!("SQL ERROR: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(certificates))
}

pub async fn update_certificate(
    Extension(state): Extension<AppState>,
    Json(payload): Json<Certificate>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query(
        r#"
            UPDATE certificates
            SET
                name = $1,
                certificate_by = $2,
                year = $3
            WHERE user_id = $4
        "#,
    )
    .bind(&payload.name)
    .bind(&payload.certificate_by)
    .bind(payload.year)
    .bind(payload.user_id)
    .execute(&state.db)
    .await
    .map_err(|e| {
        eprintln!("SQL ERROR: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::OK)
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CertificateID{
    pub id: i64,
}

pub async fn delete_certificate(
    Extension(state): Extension<AppState>,
    Json(payload): Json<CertificateID>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query(
        "
            DELETE FROM certificates
            WHERE id = $1
        ",
    )
    .bind(payload.id)
    .execute(&state.db)
    .await
    .map_err(|e| {
        eprintln!("SQL ERROR: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::OK)
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/certificates", get(get_certificates))
        .route("/certificate", post(generate_certificate))
        .route("/update-certificate", patch(update_certificate))
        .route("/delete-certificate", delete(delete_certificate))
        .layer(Extension(state))
}
