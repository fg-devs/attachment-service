use std::env;
use tonic::{Request, Response, Status};
use uuid::Uuid;
use attachments_proto::attachments_server::{Attachments};
use attachments_proto::{NewAttachmentRequest, Attachment};

pub use attachments_proto::attachments_server::AttachmentsServer;
use crate::Files;

pub mod attachments_proto {
    tonic::include_proto!("attachments");
}

#[derive(Debug, Default)]
pub struct ImplementedAttachmentsServer;

#[tonic::async_trait]
impl Attachments for ImplementedAttachmentsServer {
    async fn create_attachment(&self, request: Request<NewAttachmentRequest>) -> Result<Response<Attachment>, Status> {
        let file_url = request.into_inner().url;

        let ext = Files::get_extension(&file_url)?;
        let bytes = Files::fetch(&file_url).await?;

        let uuid = Uuid::new_v4().to_string();

        let filename = Files::create(&uuid, ext, &bytes)
            .ok()
            .ok_or_else(|| Status::invalid_argument("Unable to create file"))?;

        let url = format!("{}/{}", env::var("CDN_URI").unwrap(), filename);

        let attachment = Attachment {
            url,
            uuid,
        };

        Ok(Response::new(attachment))
    }
}

