use anyhow::Result;
use axum::body::Bytes;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs;
use std::io::Cursor;
use tonic::Status;
use tracing::*;

// (\\.[^.]+)$

lazy_static! {
    static ref FILE_EXT_RE: Regex = Regex::new("(\\.[^.]+)$").unwrap();
}

pub struct Files;

impl Files {
    pub fn setup() -> Result<()> {
        fs::create_dir("logs")?;
        fs::create_dir("files")?;

        Ok(())
    }

    pub async fn fetch(url: &String) -> Result<Bytes, Status> {
        trace!("starting fetch request");
        let response = reqwest::get(url)
            .await
            .ok()
            .ok_or_else(|| Status::invalid_argument("Unable to fetch attachment"))?;

        debug!("finished attachment fetch");

        trace!("starting bytes retrieval");
        response
            .bytes()
            .await
            .ok()
            .ok_or_else(|| Status::invalid_argument("No file in url"))
    }

    pub fn get_extension(url: &str) -> Result<&str, Status> {
        trace!("parsing extension");
        let ext = FILE_EXT_RE
            .find(url)
            .ok_or_else(|| Status::invalid_argument("Invalid file extension"))?;

        debug!("parsed file extension: {}", ext.as_str());

        Ok(ext.as_str())
    }

    pub fn create(uuid: &String, ext: &str, bytes: &Bytes) -> Result<String> {
        trace!("starting file creation");
        let filename = format!("{}{}", uuid, ext);
        let filepath = format!("files/{}", &filename);

        debug!("allocating file");
        let mut file = std::fs::File::create(&filepath)?;

        trace!("copying bytes content");
        let mut content = Cursor::new(bytes);
        std::io::copy(&mut content, &mut file)?;

        debug!("finished copying bytes content");
        info!("created file at: {}", &filepath);

        Ok(filename)
    }
}
