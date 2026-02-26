use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDate;
use utils::{db::DB, error::AppError};

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
pub struct Certificate {
    pub user_id: i64,
    pub name: String,
    pub certificate_by: String,
    pub year: NaiveDate,
}

impl Certificate {
    pub async fn generate(
       self, db: DB
    ) -> Result<(), AppError> {
        sqlx::query!(
            "
            INSERT INTO certificates
            (user_id, name, certificate_by, year)
            VALUES ($1, $2, $3, $4)",
            self.user_id,
            self.name,
            self.certificate_by,
            self.year,
        )
        .execute(&db)
        .await
        .map_err(|e| {
            eprintln!("SQL ERROR: {:?}", e);
            AppError::InternalServerError
        })?;

        Ok(())
    }

    pub async fn get(
        self, db: DB
    ) -> Result<Json<Vec<Self>>, AppError> {
        let certificates = sqlx::query_as::<_, Self>(
            r#"
            SELECT
                user_id,
                name,
                certificate_by,
                year
            FROM certificates
            WHERE user_id = $1
            "#,
        )
        .bind(self.user_id)
        .fetch_all(&db)
        .await
        .map_err(|e| {
            eprintln!("SQL ERROR: {e:?}");
            AppError::InternalServerError
        })?;

        Ok(Json(certificates))
    }

    pub async fn update(
       self, db: DB
    ) -> Result<(), AppError> {
        let result = sqlx::query!(
            r#"
                UPDATE certificates
                SET
                    name = $1,
                    certificate_by = $2,
                    year = $3
                WHERE user_id = $4
            "#,
            self.name,
            self.certificate_by,
            self.year,
            self.user_id
        )
        .execute(&db)
        .await
        .map_err(|e| {
            eprintln!("SQL ERROR: {:?}", e);
            AppError::InternalServerError
        })?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("USER".to_string()));
        }

        Ok(())
    }

    pub async fn delete(
        self, db: DB
    ) -> Result<(), AppError> {
        let result = sqlx::query(
            "
                DELETE FROM certificates
                WHERE id = $1
            ",
        )
        .bind(self.id)
        .execute(&db)
        .await
        .map_err(|e| {
            eprintln!("SQL ERROR: {:?}", e);
            AppError::InternalServerError
        })?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("USER".to_string()));
        }

        Ok(())
    }
}
