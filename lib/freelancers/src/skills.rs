use axum::Json;
use axum::http::StatusCode;
use core::str;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

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
    pub user_id: Option<i64>,
    pub skill: SkillsEnum,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateSkill {
    pub id: i64,
    pub skill: SkillsEnum,
}

impl Skills {
    pub async fn create(db: PgPool, Json(payload): Json<Skills>) -> Result<(), StatusCode> {
        let findskill = sqlx::query(
            "
            SELECT * FROM skills
            WHERE user_id = $1 AND skill = $2",
        )
        .bind(payload.user_id)
        .bind(payload.skill as SkillsEnum)
        .fetch_optional(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        if findskill.is_some() {
            return Err(StatusCode::NOT_FOUND);
        }

        sqlx::query(
            "
            INSERT INTO skills
            (user_id, skill)
            VALUES ($1, $2)",
        )
        .bind(payload.user_id)
        .bind(payload.skill as SkillsEnum)
        .execute(&db)
        .await
        .map_err(|e| {
            eprintln!("SQL ERROR: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        Ok(())
    }

    pub async fn get(db: PgPool, user_id: i64) -> Result<Json<Vec<Self>>, StatusCode> {
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
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        Ok(Json(skills))
    }

    pub async fn update(db: PgPool, Json(payload): Json<UpdateSkill>) -> Result<(), StatusCode> {
        sqlx::query(
            " UPDATE skills SET skill = $1 WHERE id = $ ",
        )
        .bind(payload.skill as SkillsEnum)
        .bind(payload.id)
        .execute(&db)
        .await
        .map_err(|e| {
            eprintln!("SQL ERROR: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        Ok(())
    }

    pub async fn delete(db: PgPool, id: i64) -> Result<(), StatusCode> {
        sqlx::query(" DELETE FROM skills WHERE id = $1 ")
            .bind(id)
            .execute(&db)
            .await
            .map_err(|e| {
                eprintln!("SQL ERROR: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

        Ok(())
    }
}
