use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utils::{
    db::DB,
    enums::{LanguageEnum, LanguageLevel},
    error::AppError,
};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Language {
    pub id: Option<i64>,
    pub user_id: Option<i64>,
    pub language: LanguageEnum,
    pub language_level: LanguageLevel,
}

impl Language {
    pub async fn add(self, db: DB) -> Result<(), AppError> {
        sqlx::query!(
            " INSERT INTO languages (user_id, language, language_level) VALUES ($1, $2, $3) ",
            self.user_id,
            self.language as LanguageEnum,
            self.language_level as LanguageLevel
        )
        .execute(&db)
        .await?;

        Ok(())
    }

    pub async fn fetch(id: i64, db: DB) -> Result<Vec<Self>, AppError> {
        let languages = sqlx::query_as!(
            Self,
            r#"
                SELECT id, user_id, language AS "language: LanguageEnum", language_level AS "language_level: LanguageLevel"
                FROM languages
                WHERE user_id = $1
            "#,
            id
        )
        .fetch_all(&db)
        .await
        .map_err(|e| {
            AppError::DatabaseError(e)
        })?;

        Ok(languages)
    }

    pub async fn delete(id: i64, db: DB) -> Result<(), AppError> {
        sqlx::query!(" DELETE FROM languages WHERE id = $1 ", id)
            .execute(&db)
            .await?;

        Ok(())
    }
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UpdateLanguage {
    pub id: i64,
    pub language: LanguageEnum,
    pub language_level: LanguageLevel,
}

impl UpdateLanguage {
    pub async fn update(self, db: DB) -> Result<(), AppError> {
        sqlx::query!(
            "
                UPDATE languages
                SET language = $2, language_level = $3
                WHERE id = $1
            ",
            self.id,
            self.language as LanguageEnum,
            self.language_level as LanguageLevel
        )
        .execute(&db)
        .await?;

        Ok(())
    }
}
