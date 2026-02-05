use axum::Extension;
use lib::AppState;
use axum::{
    Router,
    http::StatusCode,
    routing::{delete, get, post, put},
    Json,
    extract::Path,
};
use crate::handlers::{skills::{Skills, UpdateSkill}};

pub async fn add_skill(
    Extension(state): Extension<AppState>,
    Json(payload): Json<Skills>,
) -> Result<(), StatusCode> {
    Skills::generate_skill(Extension(state), Json(payload)).await
}

pub async fn get_skills(
    Extension(state): Extension<AppState>,
    Path(user_id): Path<i64>,
) -> Result<Json<Vec<Skills>>, StatusCode> {
    Skills::get_skills(Extension(state), user_id).await
}

pub async fn update_skill(
    Extension(state): Extension<AppState>,
    Json(payload): Json<UpdateSkill>,
) -> Result<(), StatusCode> {
    Skills::update_skill(Extension(state), Json(payload)).await
}

pub async fn delete_skill(
    Extension(state): Extension<AppState>,
    Path(id): Path<i64>,
) -> Result<(), StatusCode> {
    Skills::delete_skill(Extension(state), id).await
}

pub fn skills_routes(state: AppState) -> Router {
    Router::new()
        .route("/skill", post(add_skill))
        .route("/skills/{id}", get(get_skills))
        .route("/update-skill", put(update_skill))
        .route("/delete-skill/{id}", delete(delete_skill))
        .layer(Extension(state))
}