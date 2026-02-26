use serde::Deserialize;
use sqlx::query;
use utils::{db::DB, error::AppError};

#[derive(Deserialize, Debug)]
pub struct Login {
    pub email: String,
    pub password: String,
}

impl Login {
    pub async fn login(
       self, db: DB
    ) -> Result<(), AppError> {
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

        Ok(())

        // match res {
        //     Some(_) => Ok(()),
        //     None => Err(AppError::NotFound("USER".to_string())),
        // }
    }
}
