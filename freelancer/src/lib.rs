use axum::Extension;
use axum::Json;
use lib::AppState;
pub mod certificate;
pub mod routes;
pub mod skills;
use axum::{
    Router,
    http::StatusCode,
    routing::{get, post},
};
use skills::{Skills, User};

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

pub fn skills_routes(state: AppState) -> Router {
    Router::new()
        .route("/skill", post(add_skill))
        .route("/skills", get(get_skills))
        .layer(Extension(state))
}
