use core::str;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDate;
use utils::{db::DB, enums::Country, error::AppError};

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
    pub async fn create(self, db: DB) -> Result<(), AppError> {
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
        .await?;

        Ok(())
    }

    pub async fn get(db: DB, user_id: i64) -> Result<Vec<Self>, AppError> {
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
            user_id
        )
        .fetch_all(&db)
        .await
        .map_err(|e| {
            eprintln!("SQL ERROR: {e:?}");
            AppError::InternalServerError
        })?;

        Ok(educations)
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
    pub async fn update(self, db: DB) -> Result<(), AppError> {
        sqlx::query!(
            "
                UPDATE educations
                SET 
                    country = $1, 
                    degree = $2, 
                    institute = $3, 
                    major = $4, 
                    year_of_graduation = $5
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
        .await?;

        Ok(())
    }
}

pub async fn delete(db: DB, id: i64) -> Result<(), AppError> {
    sqlx::query!(" DELETE FROM educations WHERE id = $1 ", id)
        .execute(&db)
        .await
        .map_err(|e| {
            eprintln!("SQL ERROR: {:?}", e);
            AppError::InternalServerError
        })?;

    Ok(())
}
