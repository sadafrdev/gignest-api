use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utils::{db::DB, error::AppError};
use utils::enums::{LanguageEnum, LanguageLevel};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Language {
    pub id: i64,
    pub user_id: i64,
    pub language: LanguageEnum,
    pub language_level: LanguageLevel,
}

impl Language {
    pub async fn add(
       self, db: DB
    ) -> Result<(), AppError> {
        sqlx::query!(
            r#"
                INSERT INTO languages (id, user_id, language, language_level)
                VALUES ($1, $2, $3, $4)
            "#,
            self.id,
            self.user_id,
            self.language as LanguageEnum,
            self.language_level as LanguageLevel
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
        sqlx::query!(
            r#"
            DELETE FROM languages
            WHERE id = $1
            "#,
            self.id
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
    ) -> Result<Vec<Self>, AppError> {
        let languages = sqlx::query_as::<_, Self>(
            r#"
            SELECT user_id, language, language_level
            FROM languages
            WHERE user_id = $1
            "#,
        )
        .bind(self.id)
        .fetch_all(&db)
        .await
        .map_err(|e| {
            eprintln!("SQL ERROR: {:?}", e);
            AppError::InternalServerError
        })?;

        Ok(languages)
    }

    pub async fn update(
       self, db: DB
    ) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            UPDATE languages
            SET language = $2, language_level = $3
            WHERE id = $1
            "#,
            self.id,
            self.language as LanguageEnum,
            self.language_level as LanguageLevel
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
