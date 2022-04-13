use tonic::{transport::Server, Request, Response, Status};

pub mod attachments {
    tonic::include_proto!("attachments"); // The string specified here must match the proto package name
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let addr = "[::1]:50051".parse()?;

    // Server::builder()
    //     .add_service()
    //     .serve(addr)
    //     .await?;

    println!("Hello, World");

    Ok(())
}