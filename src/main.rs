use dotenv::dotenv;
use crate::services::files::Files;

mod grpc;
mod web;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    if Files::setup().is_ok() {
        // TODO: Log
    }

    grpc::start().await?;
    web::start().await?;

    Ok(())
}