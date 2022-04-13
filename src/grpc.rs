use std::env;
use anyhow::Result;
use tonic::transport::Server;
use crate::grpc::attachments::{ImplementedAttachmentsServer, AttachmentsServer};

pub(crate) mod attachments;

pub async fn start() -> Result<()>  {
    let addr = format!("[::1]:{}", env::var("GRPC_PORT")?).parse()?;
    let greeter = ImplementedAttachmentsServer::default();

    tokio::spawn(async move {
        Server::builder()
            .add_service(AttachmentsServer::new(greeter))
            .serve(addr)
            .await
            .expect("Unable to start AttachmentsServer");
    });

    println!("AttachmentsServer listening on {}", addr);

    Ok(())
}