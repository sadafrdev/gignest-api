use crate::AppState;
use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use sqlx::postgres::PgRow;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<(), StatusCode> {
    let res: Option<PgRow>= sqlx::query(
        r#"
            SELECT 
                password, email
            FROM users
            WHERE email = $1 AND password = $2
        "#,
    )
    .bind(payload.email)
    .bind(payload.password)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        eprintln!("SQL ERROR: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    match res {
        Some(_) => Ok(()),
        None => Err(StatusCode::NOT_FOUND),
    }
}
