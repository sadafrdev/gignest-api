use axum::{Json, extract::Extension, http::StatusCode};
use serde::Deserialize;
use utils::db::AppState;

#[derive(Deserialize, Debug)]
pub struct Login {
    pub email: String,
    pub password: String,
}

impl Login {
    pub async fn login(
        Extension(state): Extension<AppState>,
        Json(payload): Json<Self>,
    ) -> Result<(), StatusCode> {
        let res = sqlx::query(
            r#"
                SELECT
                   password
                FROM users
                WHERE email = $1 
            "#,
        )
        .bind(payload.email)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| {
            eprintln!("SQL ERROR: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        println!("RES: {:?}", res);
        
        Ok(())
    }
}
