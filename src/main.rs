mod grpc;
mod web;
mod services;
mod tools;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    grpc::start().await?;

    Ok(())
}