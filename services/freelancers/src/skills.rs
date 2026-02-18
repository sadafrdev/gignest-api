use axum::Extension;
use axum::{
    Json, Router,
    extract::Path,
    http::StatusCode,
    routing::{delete, get, post, put},
};
use libs::AppState;
use libs::freelancers::skills::{Skills, UpdateSkill};

pub async fn add_skill(
    Extension(state): Extension<AppState>,
    Json(payload): Json<Skills>,
) -> Result<(), StatusCode> {
    Skills::create(Extension(state), Json(payload)).await
}

pub async fn get_skills(
    Extension(state): Extension<AppState>,
    Path(user_id): Path<i64>,
) -> Result<Json<Vec<Skills>>, StatusCode> {
    Skills::get(Extension(state), user_id).await
}

pub async fn update_skill(
    Extension(state): Extension<AppState>,
    Json(payload): Json<UpdateSkill>,
) -> Result<(), StatusCode> {
    Skills::update(Extension(state), Json(payload)).await
}

pub async fn delete_skill(
    Extension(state): Extension<AppState>,
    Path(id): Path<i64>,
) -> Result<(), StatusCode> {
    Skills::delete(Extension(state), id).await
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/skill", post(add_skill))
        .route("/skills/{id}", get(get_skills))
        .route("/update-skill", put(update_skill))
        .route("/delete-skill/{id}", delete(delete_skill))
        .layer(Extension(state))
}
