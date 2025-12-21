use crate::AppState;
use axum::{extract::State, http::StatusCode, Json};
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

#[derive(Deserialize, Serialize, Debug, FromRow)]
pub struct Users {
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub email: String,
    pub phone_number: String,
    pub username: String,
    pub country: String,
    pub role: String,
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<Users>,
)-> Result<(), StatusCode> {
   sqlx::query(
    "
        INSERT INTO users 
        (first_name, last_name, password, email, phone_number, username, country, role)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
    "
   ) 
   .bind(payload.first_name)
   .bind(payload.last_name)
   .bind(payload.password)
   .bind(payload.email)
   .bind(payload.phone_number)
   .bind(payload.username)
   .bind(payload.country)
   .bind(payload.role)
   .execute(&state.db)
   .await
   .map_err(|e| {
    eprintln!("SQL ERROR: {:?}", e);
    StatusCode::INTERNAL_SERVER_ERROR
   })?;

   Ok(())
}
    