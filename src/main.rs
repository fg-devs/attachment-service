use crate::services::files::Files;
use anyhow::Result;
use dotenv::dotenv;
use tracing::*;

mod grpc;
mod services;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    trace!("setting up dotenv");
    dotenv().ok();
    services::logger::setup()?;

    trace!("starting folder setup");
    if Files::setup().is_ok() {
        debug!("created folders")
    } else {
        debug!("folders already exist")
    }
    trace!("finished folder setup");

    grpc::start().await?;
    web::start().await?;

    Ok(())
}
