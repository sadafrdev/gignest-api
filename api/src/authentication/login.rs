use authentication::login::{Login, LoginResponse};
use axum::Json;
use axum::Router;
use axum::routing::post;
use axum::extract::Extension;
use utils::{db::AppState, error::AppError};

pub async fn login(
    Extension(state): Extension<AppState>,
    Json(form): Json<Login>,
) -> Result<Json<LoginResponse>, AppError> {
    let res = form.login(state.db).await?;

    Ok(Json(res))
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/login", post(login))
        .layer(Extension(state))
}
