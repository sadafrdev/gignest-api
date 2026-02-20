use freelancers::router;
use std::net::SocketAddr;
use utils::db::establish_connection;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let state = establish_connection().await?;

    let app = router(state);

    let addr: SocketAddr = "127.0.0.1:3000".parse().unwrap();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("🚀 http://{}", addr);

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
