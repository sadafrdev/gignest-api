use lib::AppState;
use axum::Extension;
use axum::Json;
pub mod certificate;
pub mod routes;
pub mod skills;
use skills::{User, Skills};
use axum::{http::StatusCode, Router, routing::{post,get}};

pub async fn add_skill(
    Extension(state): Extension<AppState>,
    Json(payload): Json<Skills>,
) -> Result<(), StatusCode> {
    Skills::generate_skill(Extension(state), Json(payload)).await
}

pub async fn get_skills(
    Extension(state): Extension<AppState>,
    Json(payload): Json<User>,
) -> Result<Json<Option<Skills>>, StatusCode> {
    Skills::get_skills(Extension(state), Json(payload)).await
}

pub fn skills_routes(state: AppState) -> Router {
    Router::new()
    .route("/skill", post(add_skill))
    .route("/skills", get(get_skills))
    .layer(Extension(state))
}