use axum::Json;
use core::str;
use serde::{Deserialize, Serialize};
use utils::{db::DB, error::AppError};

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize, sqlx::Type)]
#[sqlx(type_name = "skills_enum", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SkillsEnum {
    WebDevelopment,
    AppDevelopment,
    DataScience,
    CyberSecurity,
    CloudComputing,
    RustProgramming,
    PythonProgramming,
    JavaProgramming,
    FrontendDevelopment,
    BackendDevelopment,
    HtmlCss,
    Javascript,
    MobileDevelopment,
    GraphicDesign,
    DigitalMarketing,
    ContentWriting,
    DataAnalysis,
    ProjectManagement,
    SeoSpecialist,
    VideoEditing,
    UiUxDesign,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Skills {
    pub user_id: i64,
    pub skill: SkillsEnum,
}

impl Skills {
    pub async fn create(
       self, db: DB
    ) -> Result<(), AppError> {
        let findskill= sqlx::query!(
            "
                SELECT * FROM skills
                WHERE user_id = $1 AND skill = $2
            ",
            self.user_id,
            self.skill as SkillsEnum
        )
        .fetch_optional(&db)
        .await
        .map_err(|_| AppError::InternalServerError)?;

        if findskill.is_some() {
            return Err(AppError::NotFound("Skill".to_string()));
        }

        sqlx::query!(
            "
            INSERT INTO skills
            (user_id, skill)
            VALUES ($1, $2)",
            self.user_id,
            self.skill as SkillsEnum
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
        let skills = sqlx::query_as::<_, Self>(
            r#"
                SELECT
                    user_id,
                    skill
                FROM skills
                WHERE user_id = $1
            "#,
        )
        .bind(self.user_id)
        .fetch_all(&db)
        .await
        .map_err(|e| {
            eprintln!("SQL ERROR: {:?}", e);
            AppError::InternalServerError
        })?;

        Ok(Json(skills))
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateSkill {
    pub id: i64,
    pub skill: SkillsEnum,
}

impl UpdateSkill {
    pub async fn update(
       self, db: DB
    ) -> Result<(), AppError> {
        sqlx::query!(
            "
                UPDATE skills
                SET skill = $1
                WHERE id = $2
            ",
            self.skill as SkillsEnum,
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
}


#[derive(Deserialize, Serialize, Debug)]
pub struct DeleteSkill {
    pub id: i64,
    pub skill: SkillsEnum,
}

impl DeleteSkill{

    pub async fn delete(self, db: DB) -> Result<(), AppError> {
        sqlx::query!(
            "
                DELETE FROM skills
                WHERE id = $1
            ",
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
}
