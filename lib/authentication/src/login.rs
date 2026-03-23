use axum::Json;
use serde::Deserialize;
use sqlx::query;
use utils::{db::DB, error::AppError, jwt::create_jwt, security::verify};
use serde::Serialize;

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Deserialize, Debug)]
pub struct Login {
    pub email: String,
    pub password: String,
}

impl Login {
    pub async fn login(self, db: DB) -> Result<Json<LoginResponse>, AppError> {
        let res = query!(
            " SELECT id, password FROM users WHERE email = $1 ",
            self.email
        )
        .fetch_optional(&db)
        .await?
        .ok_or(AppError::InternalServerError)?;

        verify(&res.password, self.password)?;
        
        let token = create_jwt(res.id)
            .map_err(|_| AppError::InternalServerError)?;
        
        Ok(Json(LoginResponse { token }))
    }
}
