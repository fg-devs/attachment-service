use std::fs;
use std::io::Cursor;
use anyhow::Result;
use axum::body::Bytes;
use lazy_static::lazy_static;
use regex::Regex;
use tonic::Status;

// (\\.[^.]+)$

lazy_static! {
    static ref FILE_EXT_RE: Regex = Regex::new("(\\.[^.]+)$").unwrap();
}

pub struct Files;

impl Files {
    pub fn setup() -> Result<()> {
        fs::create_dir("files")?;

        Ok(())
    }

    pub async fn fetch(url: &String) -> Result<Bytes, Status> {
        let response = reqwest::get(url).await.ok().ok_or_else(|| Status::invalid_argument("Unable to fetch attachment"))?;
        response.bytes().await.ok().ok_or_else(|| Status::invalid_argument("No file in url"))
    }

    pub fn get_extension(url: &str) -> Result<&str, Status> {
        let ext = FILE_EXT_RE.find(url).ok_or_else(|| Status::invalid_argument("Invalid file extension"))?;
        Ok(ext.as_str())
    }

    pub fn create(uuid: &String, ext: &str, bytes: &Bytes) -> Result<String> {
        let filename = format!("{}{}", uuid, ext);
        let mut file = std::fs::File::create(format!("files/{}", &filename))?;
        let mut content =  Cursor::new(bytes);
        std::io::copy(&mut content, &mut file)?;

        Ok(filename)
    }
}