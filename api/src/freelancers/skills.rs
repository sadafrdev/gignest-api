use axum::{
    Extension, Json, Router,
    extract::Path,
    routing::{delete, get, post, put},
};
use utils::{db::DB, error::AppError};
use freelancers::skills::{Skills, SkillUserID, Skill};

pub async fn add_skill(
    Extension(db): Extension<DB>,
    Json(form): Json<Skills>,
) -> Result<(), AppError> {
    form.create(db).await
}

pub async fn get_skills(
    Path(id): Path<SkillUserID>,
    Extension(db): Extension<DB>,
) -> Result<Json<Vec<Skills>>, AppError> {
    id.get(db).await.map(Json)
}

pub async fn update_skill(
    Extension(db): Extension<DB>,
    Json(form): Json<Skill>,
) -> Result<(), AppError> {
    form.update(db).await
}

pub async fn delete_skill(
    Path(id): Path<Skill>,
    Extension(db): Extension<DB>,
) -> Result<(), AppError> {
    id.delete(db).await
}

pub fn router() -> Router {
    Router::new()
        .route("/skill", post(add_skill))
        .route("/skills/{id}", get(get_skills))
        .route("/skill/update", put(update_skill))
        .route("/skill/delete/{id}", delete(delete_skill))
}
