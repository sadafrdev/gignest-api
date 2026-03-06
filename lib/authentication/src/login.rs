use serde::Deserialize;
use sqlx::query;
use utils::{db::DB, error::AppError};
use serde::Serialize;
use jsonwebtoken::{encode, EncodingKey, Header};
use chrono::{Utc, Duration};

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: i64,
    exp: usize,
}

fn create_jwt(user_id: i64) -> Result<String, AppError> {
    const SECRET: &[u8] = b"123456";

    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(1))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET),
    )
    .map_err(|_| AppError::InternalServerError)
}

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
    pub async fn login(self, db: DB) -> Result<LoginResponse, AppError> {
        let res = query!(
            "
            SELECT id, password
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

        let user = match res {
            Some(u) => u,
            None => return Err(AppError::NotFound("USER".to_string())),
        };

        if user.password != self.password {
            return Err(AppError::Unauthorized);
        }

        let token = create_jwt(user.id)?;

        Ok(LoginResponse { token })
    }
}
