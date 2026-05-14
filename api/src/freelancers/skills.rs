use axum::{
    Extension, Json, Router,
    extract::Path,
    routing::{delete, get, post, put},
};
use freelancers::skills::{Skill, SkillsEnum, UpdateSkill};
use utils::{db::DB, error::AppError};

pub async fn add_skill(
    Extension(db): Extension<DB>,
    Json(form): Json<Skill>,
) -> Result<(), AppError> {
    form.create(db).await
}

pub async fn get_skills(
    Path(user_id): Path<i64>,
    Extension(db): Extension<DB>,
) -> Result<Json<Vec<Skill>>, AppError> {
    Skill::get(db, user_id).await.map(Json)
}

pub async fn update_skill(
    Extension(db): Extension<DB>,
    Json(form): Json<UpdateSkill>,
) -> Result<(), AppError> {
    form.update(db).await
}

pub async fn delete_skill(
    Path((id, skill)): Path<(i64, SkillsEnum)>,
    Extension(db): Extension<DB>,
) -> Result<(), AppError> {
    Skill::delete(db, id, skill).await
}

pub fn router() -> Router {
    Router::new()
        .route("/skill", post(add_skill))
        .route("/skills/{id}", get(get_skills))
        .route("/skill", put(update_skill))
        .route("/skill/{id}", delete(delete_skill))
}
