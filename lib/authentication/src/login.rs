use axum::{http::StatusCode};
use serde::Deserialize;
use sqlx::query;
use utils::db::DB;

#[derive(Deserialize, Debug)]
pub struct Login {
    pub email: String,
    pub password: String,
}

impl Login {
    pub async fn login(
       self, db: DB
    ) -> Result<(), StatusCode> {
        let res = query!(
            "
                SELECT
                   password
                FROM users
                WHERE email = $1 
            ",
            self.email
        )
        .fetch_optional(&db)
        .await
        .map_err(|e| {
            eprintln!("SQL ERROR: {:?}", e);
            AppError::InternalServerError
        })?;

        match res {
            Some(_) => Ok(()),
            None => Err(AppError::NotFound("USER".to_string())),
        }
    }
}
