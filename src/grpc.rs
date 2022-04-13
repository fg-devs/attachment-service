use anyhow::Result;
use tonic::transport::Server;
use crate::grpc::attachments::{ImplementedAttachmentsServer, AttachmentsServer};

mod attachments;

pub async fn start() -> Result<()>  {
    let addr = "[::1]:50051".parse()?;
    let greeter = ImplementedAttachmentsServer::default();

    println!("AttachmentsServer listening on {}", addr);

    Server::builder()
        .add_service(AttachmentsServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}