use axum::http::StatusCode;
use axum::{Extension, Json};
use axum::{
    Router,
    routing::{delete, get, post, put},
};
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

pub async fn generate_skill(
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

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub user_id: i64,
}

pub async fn get_skills(
    Extension(state): Extension<AppState>,
    Json(payload): Json<User>,
) -> Result<Json<Vec<Skills>>, StatusCode> {
    let skills = sqlx::query_as::<_, Skills>(
        r#"
        SELECT
            user_id,
            skill
        FROM skills
        WHERE user_id = $1
        "#,
    )
    .bind(payload.user_id)
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        eprintln!("SQL ERROR: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(skills))
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateSkill {
    pub id: i64,
    pub skill: SkillsEnum,
}

pub async fn update_skill(
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

#[derive(Deserialize, Serialize, Debug)]
pub struct SkillID {
    pub id: i64,
}

pub async fn delete_skill(
    Extension(state): Extension<AppState>,
    Json(payload): Json<SkillID>,
) -> Result<(), StatusCode> {
    sqlx::query(
        "
        DELETE FROM skills
        WHERE id = $1",
    )
    .bind(payload.id)
    .execute(&state.db)
    .await
    .map_err(|e| {
        eprintln!("SQL ERROR: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(())
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/skill", post(generate_skill))
        .route("/skills", get(get_skills))
        .route("/update-skill", put(update_skill))
        .route("/delete-skill", delete(delete_skill))
        .layer(Extension(state))
}
