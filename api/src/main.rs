use std::net::SocketAddr;
use utils::{ENV, db::establish_connection};
mod authentication;
pub mod clients;
pub mod freelancers;
pub mod routes;
pub mod reviews;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    ENV::load_file();

    let connection = establish_connection().await?;
    let app = routes::router(connection );

    let addr: SocketAddr = "127.0.0.1:3000".parse().unwrap();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("🚀 http://{}", addr);

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
 