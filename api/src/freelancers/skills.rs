use axum::{
    Extension, Json, Router,
    extract::Path,
    routing::{delete, get, post, put},
};
use sqlx::PgPool;
use utils::{db::AppState, error::AppError};
use freelancers::skills::{Skills, UpdateSkill, DeleteSkill};

pub async fn add_skill(
    Extension(db): Extension<PgPool>,
    Json(form): Json<Skills>,
) -> Result<(), AppError> {
    form.create(db).await
}

pub async fn get_skills(
    Path(user_id): Path<i64>,
    Extension(db): Extension<PgPool>,
) -> Result<Json<Vec<Skills>>, AppError> {
    Skills::get(db).await.map(Json)
}

pub async fn update_skill(
    Extension(db): Extension<PgPool>,
    Json(form): Json<UpdateSkill>,
) -> Result<(), AppError> {
    form.update(db).await
}

pub async fn delete_skill(
    Path(id): Path<i64>,
    Extension(db): Extension<PgPool>,
) -> Result<(), AppError> {
    DeleteSkill::delete(db).await
}

pub fn router() -> Router {
    Router::new()
        .route("/skill", post(add_skill))
        .route("/skills/{id}", get(get_skills))
        .route("/update-skill", put(update_skill))
        .route("/delete-skill/{id}", delete(delete_skill))
}
