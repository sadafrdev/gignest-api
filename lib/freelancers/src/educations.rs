use axum::Json;
use core::str;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDate;
use utils::{db::DB, error::AppError};
use utils::enums::Country;

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
pub struct Education {
    pub user_id: i64,
    pub country: Country,
    pub degree: String,
    pub institute: String,
    pub major: String,
    pub year_of_graduation: NaiveDate,
}

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
pub struct UpdateEducation {
    pub id: i64,
    pub country: Country,
    pub degree: String,
    pub institute: String,
    pub major: String,
    pub year_of_graduation: NaiveDate,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DeleteEducation {
    pub id: i64,
}

impl Education {
    pub async fn create(
       self, db: DB
    ) -> Result<(), AppError> {
        sqlx::query!(
            "
            INSERT INTO educations
            (user_id, country, degree, institute, major, year_of_graduation)
            VALUES ($1, $2, $3, $4, $5, $6)",
            self.user_id,
            self.country as Country,
            self.degree,
            self.institute,
            self.major,
            self.year_of_graduation
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
        let educations = sqlx::query_as::<_, Self>(
            r#"
                SELECT
                    user_id,
                    country,
                    degree,
                    institute,
                    major,
                    year_of_graduation
                FROM educations
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

        Ok(Json(educations))
    }

    pub async fn update(
        self, db: DB
    ) -> Result<(), AppError> {
        sqlx::query!(
            "
                UPDATE educations
                SET country = $1, degree = $2, institute = $3, major = $4, year_of_graduation = $5
                WHERE id = $6
            ",
            self.country as Country,
            self.degree,
            self.institute,
            self.major,
            self.year_of_graduation,
            self.id,
        )
        .execute(&db)
        .await
        .map_err(|e| {
            eprintln!("SQL ERROR: {:?}", e);
            AppError::InternalServerError
        })?;

        Ok(())
    }

    pub async fn delete(
       self, db: DB
    ) -> Result<(), AppError> {
        let result = sqlx::query(
            "
                DELETE FROM educations
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
            return Err(AppError::NotFound("Education".to_string()));
        }

        Ok(())
    }
}
