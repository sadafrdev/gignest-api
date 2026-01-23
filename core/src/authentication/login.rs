use axum::{http::StatusCode};
use crate::AppState;
use serde::Deserialize;
use crate::DB;

#[derive(Deserialize)]
pub struct Login{
    pub email: String,
    pub password: String,
}

impl Login {
    pub async fn login(self, db: DB) -> Result<(), StatusCode> {
        let res = sqlx::query!(
            r#"
                SELECT
                    password, email
                FROM users
                WHERE email = $1 AND password = $2
            "#,
            self.email,
            self.password,
        )
        .fetch_optional(&db)
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
}
