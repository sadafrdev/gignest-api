use axum::Json;
use core::str;
use serde::{Deserialize, Serialize};
use sqlx::{types::chrono::NaiveDate, PgPool};
use utils::{error::AppError, enums::Country};

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
pub struct Education {
    pub user_id: Option<i64>,
    pub country: Country,
    pub degree: String,
    pub institute: String,
    pub major: String,
    pub year_of_graduation: NaiveDate,
}

impl Education {
    pub async fn create(
       self, db: PgPool
    ) -> Result<(), AppError> {
        sqlx::query!(
            " INSERT INTO educations (user_id, country, degree, institute, major, year_of_graduation) VALUES ($1, $2, $3, $4, $5, $6)",
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

    pub async fn get(db: PgPool, id: i64) -> Result<Json<Vec<Self>>, AppError> {
        let educations = sqlx::query_as!(
            Self,
            r#"
                SELECT
                    user_id,
                    country AS "country: Country",
                    degree,
                    institute,
                    major,
                    year_of_graduation
                FROM educations
                WHERE user_id = $1
            "#,
            id
        )
        .fetch_all(&db)
        .await
        .map_err(|e| {
            eprintln!("SQL ERROR: {e:?}");
            AppError::InternalServerError
        })?;

        Ok(Json(educations))
    }

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

impl UpdateEducation {
    pub async fn update(
        self, db: PgPool
    ) -> Result<(), AppError> {
        sqlx::query!(
            "
                UPDATE educations
                SET 
                    country = $2, 
                    degree = $3, 
                    institute = $4, 
                    major = $5, 
                    year_of_graduation = $6
                WHERE id = $1
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

}

#[derive(Deserialize, Serialize, Debug)]
pub struct DeleteEducation {
    pub id: i64,
}

impl DeleteEducation {

    pub async fn delete(
       self, db: PgPool
    ) -> Result<(), AppError> {
        let result = sqlx::query!(
            " DELETE FROM educations WHERE id = $1 ",
            self.id
        )
        .execute(&db)
        .await
        .map_err(|e| {
            eprintln!("SQL ERROR: {:?}", e);
            AppError::InternalServerError
        })?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Education"));
        }

        Ok(())
    }
}
