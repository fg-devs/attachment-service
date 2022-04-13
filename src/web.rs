use std::env;
use std::net::SocketAddr;
use anyhow::Result;
use axum::http::StatusCode;
use axum::Router;
use axum::routing::get_service;
use tower_http::services::ServeDir;

pub async fn start() -> Result<()> {
    let app = Router::new()
        .nest("/", get_service(ServeDir::new("files")).handle_error(|error: std::io::Error| async move {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unhandled internal error: {}", error),
            )
        }));

    let addr = SocketAddr::from(([127, 0, 0, 1], env::var("HTTP_PORT").unwrap().parse::<u16>().unwrap()));

   axum::Server::bind(&addr)
       .serve(app.into_make_service())
       .await
       .expect("Unable to start HTTP Server");

    Ok(())
}