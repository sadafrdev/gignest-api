use crate::AppState;
use axum::http::StatusCode;
use axum::{extract::Json, extract::State};
use serde::Deserialize;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Deserialize, Serialize, Debug, FromRow)]
pub struct User {
    pub password: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<User>, StatusCode> {
    let rows = sqlx::query_as::<_, User>(
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

    match rows {
        Some(user) => Ok(Json(user)),
        None => Err(StatusCode::NOT_FOUND),
    }
}
