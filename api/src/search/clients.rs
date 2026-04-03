use axum::{Extension, Json, Router};
use axum::routing::get;
use utils::{db::DB, error::AppError};
use search::clients::{Clients, SearchClients};

pub async fn search_clients(
    Extension(db): Extension<DB>,
    Json(form): Json<SearchClients>,
)  -> Result<Json<Vec<Clients>>, AppError> {
    form.search(db).await.map(Json)
}

pub fn router() -> Router {
    Router::new()
        .route("/client/search", get(search_clients))
}