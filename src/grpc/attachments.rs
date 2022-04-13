use tonic::{Request, Response, Status};
use attachments_proto::attachments_server::{Attachments};
use attachments_proto::{NewAttachmentRequest, RetrieveAttachmentRequest, AttachmentReply, Attachment, Uuid};

pub use attachments_proto::attachments_server::AttachmentsServer;

pub mod attachments_proto {
    tonic::include_proto!("attachments"); // The string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct ImplementedAttachmentsServer;

#[tonic::async_trait]
impl Attachments for ImplementedAttachmentsServer {
    async fn create_attachment(&self, request: Request<NewAttachmentRequest>) -> Result<Response<AttachmentReply>, Status> {
        println!("Got a request: {:?}", request);


        let reply = AttachmentReply {
            attachment: Attachment {
                url: String::from("test"),
                uuid: Uuid {
                    value: String::from("test"),
                }.into(),
            }.into(),
        };

        Ok(Response::new(reply))
    }

    async fn retrieve_attachment(&self, request: Request<RetrieveAttachmentRequest>) -> Result<Response<AttachmentReply>, Status> {
        println!("Got a request: {:?}", request);


        let reply = AttachmentReply {
            attachment: Attachment {
                url: String::from("test"),
                uuid: Uuid {
                    value: String::from("test"),
                }.into(),
            }.into(),
        };

        Ok(Response::new(reply))
    }
}

