use core::str;
use serde::{Deserialize, Serialize};
use utils::{db::DB, error::AppError};

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize, sqlx::Type)]
#[sqlx(type_name = "skills_enum", rename_all = "PascalCase")]
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
    SeoSpeciallities,
    VideoEditing,
    UiUxDesign,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Skill {
    pub user_id: Option<i64>,
    pub skill: SkillsEnum,
}

impl Skill {
    pub async fn create(self, db: DB) -> Result<(), AppError> {
        sqlx::query!(
            "
                SELECT user_id 
                FROM skills 
                WHERE user_id = $1 AND skill = $2::skills_enum
            ",
            self.user_id,
            self.skill as SkillsEnum
        )
        .fetch_optional(&db)
        .await?
        .ok_or(AppError::InternalServerError)?;

        sqlx::query!(
            " INSERT INTO skills (user_id, skill) VALUES ($1, $2) ",
            self.user_id,
            self.skill as SkillsEnum
        )
        .execute(&db)
        .await?;

        Ok(())
    }

    pub async fn get(db: DB, user_id: i64) -> Result<Vec<Self>, AppError> {
        let skills = sqlx::query_as!(
            Self,
            r#"
                SELECT
                    user_id,
                    skill AS "skill: SkillsEnum"
                FROM skills 
                WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_all(&db)
        .await
        .map_err(|e| {
            eprintln!("SQL ERROR: {:?}", e);
            AppError::InternalServerError
        })?;

        Ok(skills)
    }

    pub async fn delete(db: DB, id: i64, skill: SkillsEnum) -> Result<(), AppError> {
        sqlx::query!(
            " DELETE FROM skills  WHERE id = $1 and skill = $2 ",
            id,
            skill as SkillsEnum
        )
        .execute(&db)
        .await?;
    
        Ok(())
    }

}

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateSkill {
    pub id: i64,
    pub skill: SkillsEnum,
}

impl UpdateSkill {
    pub async fn update(self, db: DB) -> Result<(), AppError> {
        sqlx::query!(
            " UPDATE skills SET skill = $1 WHERE id = $2 ",
            self.skill as SkillsEnum,
            self.id
        )
        .execute(&db)
        .await?;

        Ok(())
    }
}
