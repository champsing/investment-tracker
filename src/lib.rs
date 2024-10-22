mod database;
mod error;

// pub mod auth;
// pub mod investment;
// pub mod transaction;

use actix_files::NamedFile;
use actix_web::Responder;
use error::ServerError;
use std::fs;

pub fn init() -> Result<(), ServerError> {
    fs::create_dir_all("data/")?;
    fs::create_dir_all("dist/")?;

    database::init()?;

    // auth::init()?;
    // transaction::init()?;

    Ok(())
}

pub async fn index() -> Result<impl Responder, ServerError> {
    NamedFile::open_async("dist/index.html")
        .await
        .map_err(|e| e.into())
}
