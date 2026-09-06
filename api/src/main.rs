use std::net::SocketAddr;
use utils::{ENV, db::establish_connection};
mod authentication;
pub mod clients;
pub mod freelancers;
pub mod reviews;
pub mod routes;
pub mod search;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    ENV::load_file();

    let connection = establish_connection().await?;
    let app = routes::router(connection);

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().unwrap();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("🚀 http://{}", addr);

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
