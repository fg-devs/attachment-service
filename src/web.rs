use anyhow::Result;
use axum::http::StatusCode;
use axum::routing::get_service;
use axum::Router;
use std::env;
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing::info;

pub async fn start() -> Result<()> {
    let app = Router::new().nest(
        "/",
        get_service(ServeDir::new("files"))
            .handle_error(|error: std::io::Error| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", error),
                )
            })
            .layer(TraceLayer::new_for_http()),
    );

    let addr = SocketAddr::from((
        [0, 0, 0, 0],
        env::var("HTTP_PORT").unwrap().parse::<u16>().unwrap(),
    ));

    info!("starting HTTP server...");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Unable to start HTTP Server");

    Ok(())
}
