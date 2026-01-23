use axum::Extension;
use axum::Json;
use lib::AppState;
pub mod certificate;
pub mod routes;
pub mod skills;
use axum::{
    Router,
    http::StatusCode,
    routing::{delete, get, post, put},
};
use skills::{SkillID, Skills, UpdateSkill, User};

pub async fn add_skill(
    Extension(state): Extension<AppState>,
    Json(payload): Json<Skills>,
) -> Result<(), StatusCode> {
    Skills::generate_skill(Extension(state), Json(payload)).await
}

pub async fn get_skills(
    Extension(state): Extension<AppState>,
    Json(payload): Json<User>,
) -> Result<Json<Vec<Skills>>, StatusCode> {
    Skills::get_skills(Extension(state), Json(payload)).await
}

pub async fn update_skill(
    Extension(state): Extension<AppState>,
    Json(payload): Json<UpdateSkill>,
) -> Result<(), StatusCode> {
    Skills::update_skill(Extension(state), Json(payload)).await
}

pub async fn delete_skill(
    Extension(state): Extension<AppState>,
    Json(payload): Json<SkillID>,
) -> Result<(), StatusCode> {
    Skills::delete_skill(Extension(state), Json(payload)).await
}

pub fn skills_routes(state: AppState) -> Router {
    Router::new()
        .route("/skill", post(add_skill))
        .route("/skills", get(get_skills))
        .route("/update-skill", put(update_skill))
        .route("/delete-skill", delete(delete_skill))
        .layer(Extension(state))
}
