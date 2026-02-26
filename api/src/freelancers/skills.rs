use axum::Extension;
use axum::{
    Json, Router,
    extract::Path,
    routing::{delete, get, post, put},
};
use freelancers::skills::{Skills, UpdateSkill};
use utils::{db::AppState, error::AppError};

pub async fn add_skill(
    Extension(state): Extension<AppState>,
    Json(form): Json<Skills>,
) -> Result<(), AppError> {
    form.create(state.db).await
}

pub async fn get_skills(
    Extension(state): Extension<AppState>,
    Path(user_id): Path<i64>,
) -> Result<Json<Vec<Skills>>, AppError> {
    Skills::get(state.db).await
}

pub async fn update_skill(
    Extension(state): Extension<AppState>,
    Json(form): Json<UpdateSkill>,
) -> Result<(), AppError> {
    form.update(state.db).await
}

pub async fn delete_skill(
    Extension(state): Extension<AppState>,
    Path(id): Path<i64>,
) -> Result<(), AppError> {
    Skills::delete(state.db).await
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/skill", post(add_skill))
        .route("/skills/{id}", get(get_skills))
        .route("/update-skill", put(update_skill))
        .route("/delete-skill/{id}", delete(delete_skill))
        .layer(Extension(state))
}
