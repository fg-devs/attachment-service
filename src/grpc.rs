use crate::grpc::attachments::{AttachmentsServer, ImplementedAttachmentsServer};
use anyhow::Result;
use std::env;
use tonic::transport::Server;
use tracing::*;

pub(crate) mod attachments;

pub async fn start() -> Result<()> {
    let addr = format!("[::0]:{}", env::var("GRPC_PORT")?).parse()?;
    let greeter = ImplementedAttachmentsServer::default();

    tokio::spawn(async move {
        info!("starting gRPC server...");
        Server::builder()
            .trace_fn(|_| tracing::info_span!("attachments_server"))
            .add_service(AttachmentsServer::new(greeter))
            .serve(addr)
            .await
            .expect("Unable to start AttachmentsServer");
    });

    Ok(())
}
