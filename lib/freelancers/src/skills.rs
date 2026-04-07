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
pub struct Skills {
    pub user_id: Option<i64>,
    pub skill: SkillsEnum,
}

impl Skills {
    pub async fn create(
       self, db: DB
    ) -> Result<(), AppError> {
        let findskill = sqlx::query!(
            "
                SELECT user_id 
                FROM skills 
                WHERE user_id = $1 AND skill = $2::skills_enum
            ",
            self.user_id,
            self.skill as SkillsEnum
        )
        .fetch_optional(&db)
        .await?;

        if findskill.is_some() {
            return Err(AppError::NotFound("Skill"));
        }

        sqlx::query!(
            " INSERT INTO skills (user_id, skill) VALUES ($1, $2) ",
            self.user_id,
            self.skill as SkillsEnum
        )
        .execute(&db)
        .await?;

        Ok(())
    }

}

#[derive(Deserialize, Serialize, Debug)]
pub struct Skill {
    pub id: i64,
    pub skill: SkillsEnum,
}

impl Skill {
    pub async fn update(
       self, db: DB
    ) -> Result<(), AppError> {
        sqlx::query!(
            " UPDATE skills SET skill = $1 WHERE id = $2 ",
            self.skill as SkillsEnum,
            self.id
        )
        .execute(&db)
        .await?;

        Ok(())
    }

    pub async fn delete(self, db: DB) -> Result<(), AppError> {
        sqlx::query!(
            " DELETE FROM skills  WHERE id = $1 and skill = $2 ",
            self.id,
            self.skill as SkillsEnum
        )
        .execute(&db)
        .await?;

        Ok(())
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SkillUserID {
    pub user_id: i64,
}

impl SkillUserID{
    pub async fn get(self, db: DB) -> Result<Vec<Skills>, AppError> {
        let skills = sqlx::query_as!(
            Skills,
            r#"
                SELECT
                    user_id,
                    skill AS "skill: SkillsEnum"
                FROM skills 
                WHERE user_id = $1
            "#,
            self.user_id
        )
        .fetch_all(&db)
        .await
        .map_err(|e| {
            eprintln!("SQL ERROR: {:?}", e);
            AppError::InternalServerError
        })?;

        Ok(skills)
    }
}