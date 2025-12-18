mod authentication;
mod routes;
use dotenvy::dotenv;
use sqlx::PgPool;
use std::net::SocketAddr;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL is missing. Put it in .env or export it before running.");

    let db = PgPool::connect(&database_url).await?;

    let state = AppState { db };

    let app = routes::create_routes(state);

    let addr: SocketAddr = "127.0.0.1:3000".parse().unwrap();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("🚀 http://{}", addr);
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
