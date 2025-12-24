use axum::Extension;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenvy::dotenv().ok();

    let pool = db::connect().await?;

    let app = gateway::router().layer(Extension(pool));

    let addr: SocketAddr = "127.0.0.1:3000".parse().unwrap();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("🚀 http://{}", addr);

    // ✅ Axum 0.7 way
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
