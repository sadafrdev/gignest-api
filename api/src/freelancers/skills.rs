use axum::{
    Extension, Json, Router,
    extract::Path,
    http::StatusCode,
    routing::{delete, get, post, put},
};
use freelancers::skills::{Skills, UpdateSkill};
use sqlx::PgPool;

pub async fn add_skill(
    Extension(db): Extension<PgPool>,
    Json(payload): Json<Skills>,
) -> Result<(), StatusCode> {
    Skills::create(db, Json(payload)).await
}

pub async fn get_skills(
    Extension(db): Extension<PgPool>,
    Path(user_id): Path<i64>,
) -> Result<Json<Vec<Skills>>, StatusCode> {
    Skills::get(db, user_id).await
}

pub async fn update_skill(
    Extension(db): Extension<PgPool>,
    Json(payload): Json<UpdateSkill>,
) -> Result<(), StatusCode> {
    Skills::update(db, Json(payload)).await
}

pub async fn delete_skill(
    Path(id): Path<i64>,
    Extension(db): Extension<PgPool>,
) -> Result<(), StatusCode> {
    Skills::delete(db, id).await
}

pub fn router() -> Router {
    Router::new()
        .route("/skill", post(add_skill))
        .route("/skills/{id}", get(get_skills))
        .route("/update-skill", put(update_skill))
        .route("/delete-skill/{id}", delete(delete_skill))
}
