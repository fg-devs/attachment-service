use anyhow::Result;
use axum::Router;
use std::env;
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use tracing::info;

pub async fn start() -> Result<()> {
    let app = Router::new().nest_service("/", ServeDir::new("files"));

    let addr = SocketAddr::from((
        [0, 0, 0, 0],
        env::var("HTTP_PORT").unwrap().parse::<u16>().unwrap(),
    ));

    info!("starting HTTP server...");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect(format!("Unable to bind: {}", addr).as_str());
    axum::serve(listener, app)
        .await
        .expect("Unable to start HTTP Server");

    Ok(())
}
