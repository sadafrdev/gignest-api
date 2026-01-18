use serde::{Deserialize, Serialize};
use axum::{Extension, Json};
use lib::AppState;
use axum::{
    Router,
    routing::post,
};
use axum::http::StatusCode;

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

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/skill", post(generate_skill))
        .layer(Extension(state))
}