use axum::http::StatusCode;
use axum::{Extension, Json};
use core::str;
use lib::AppState;
use serde::{Deserialize, Serialize};

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

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateSkill {
    pub id: i64,
    pub skill: SkillsEnum,
}

impl Skills {
    pub async fn create(
        Extension(state): Extension<AppState>,
        Json(payload): Json<Skills>,
    ) -> Result<(), StatusCode> {
        let findskill = sqlx::query(
            "
            SELECT * FROM skills
            WHERE user_id = $1 AND skill = $2",
        )
        .bind(payload.user_id)
        .bind(payload.skill as SkillsEnum)
        .fetch_optional(&state.db)
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
        .execute(&state.db)
        .await
        .map_err(|e| {
            eprintln!("SQL ERROR: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        Ok(())
    }

    pub async fn get(
        Extension(state): Extension<AppState>,
        user_id: i64,
    ) -> Result<Json<Vec<Self>>, StatusCode> {
        let skills = sqlx::query_as::<_, Self>(
            r#"
                SELECT
                    user_id,
                    skill
                FROM skills
                WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_all(&state.db)
        .await
        .map_err(|e| {
            eprintln!("SQL ERROR: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        Ok(Json(skills))
    }

    pub async fn update(
        Extension(state): Extension<AppState>,
        Json(payload): Json<UpdateSkill>,
    ) -> Result<(), StatusCode> {
        sqlx::query(
            "
            UPDATE skills
            SET skill = $1
            WHERE id = $2",
        )
        .bind(payload.skill as SkillsEnum)
        .bind(payload.id)
        .execute(&state.db)
        .await
        .map_err(|e| {
            eprintln!("SQL ERROR: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        Ok(())
    }

    pub async fn delete(Extension(state): Extension<AppState>, id: i64) -> Result<(), StatusCode> {
        sqlx::query(
            "
            DELETE FROM skills
            WHERE id = $1",
        )
        .bind(id)
        .execute(&state.db)
        .await
        .map_err(|e| {
            eprintln!("SQL ERROR: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        Ok(())
    }
}
