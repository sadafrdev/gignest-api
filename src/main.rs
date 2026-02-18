use libs;
pub mod router;
use libs::AppState;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL is missing. Put it in .env or export it before running.");

    let database_url = &database_url;
    let db = sqlx::PgPool::connect(&database_url).await?;

    let state = AppState { db };

    let app = router::router(state.clone());

    let addr: SocketAddr = "127.0.0.1:3000".parse().unwrap();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("🚀 http://{}", addr);

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
