use attachments_proto::attachments_server::Attachments;
use attachments_proto::{Attachment, NewAttachmentRequest};
use std::env;
use tonic::{Request, Response, Status};
use tracing::*;
use uuid::Uuid;
use url::Url;

use crate::Files;
pub use attachments_proto::attachments_server::AttachmentsServer;

pub mod attachments_proto {
    tonic::include_proto!("attachments");
}

#[derive(Debug, Default)]
pub struct ImplementedAttachmentsServer;

#[tonic::async_trait]
impl Attachments for ImplementedAttachmentsServer {
    async fn create_attachment(
        &self,
        request: Request<NewAttachmentRequest>,
    ) -> Result<Response<Attachment>, Status> {
        info!("received create attachment request");

        trace!("parsing url");
        let mut file_url = request.into_inner().url;

        // Parse the URL and strip off the query parameters
        if let Ok(mut parsed_url) = Url::parse(&file_url) {
            parsed_url.set_query(None);
            file_url = parsed_url.to_string();
        }

        debug!("finished parsing url: {}", file_url);

        let ext = Files::get_extension(&file_url)?;
        let bytes = Files::fetch(&file_url).await?;
        info!("fetched attachment: {}", file_url);

        let uuid = Uuid::new_v4().to_string();

        let filename = Files::create(&uuid, ext, &bytes)
            .ok()
            .ok_or_else(|| Status::invalid_argument("Unable to create file"))?;

        let url = format!("{}/{}", env::var("CDN_URI").unwrap(), filename);

        let attachment = Attachment { url, uuid };

        tracing::debug!("sending response");

        Ok(Response::new(attachment))
    }
}
