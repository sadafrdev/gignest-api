use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Invalid input data: {0}")]
    ValidationError(String),
    #[error("Resource not found: {0}")]
    NotFound(String),
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Internal server error")]
    InternalServerError,
    #[error("Unauthorized")]
    UNAUTHORIZED,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::DatabaseError(err) => {
                eprintln!("Database error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Database operation failed".to_string(),
                )
            }
            AppError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "An unexpected error occurred".to_string(),
            ),
            AppError::UNAUTHORIZED => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
        };

        let body = Json(ErrorResponse {
            message: error_message,
        });

        (status, body).into_response()
    }
}
