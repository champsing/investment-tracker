pub mod auth;
pub mod constant;
mod error;
pub mod investment;
pub mod transaction;

use actix_files::NamedFile;
use actix_web::Responder;
use const_format::formatcp as const_format;
pub use error::Result;
use std::fs;

pub fn init() -> Result<()> {
    fs::create_dir_all(constant::path::CACHE)?;
    fs::create_dir_all(constant::path::DATA)?;
    fs::create_dir_all(constant::path::STATIC)?;

    auth::init()?;
    transaction::init()?;

    Ok(())
}

pub async fn index() -> Result<impl Responder> {
    const INDEX: &str = const_format!("{}/index.html", constant::path::STATIC);
    NamedFile::open_async(INDEX).await.map_err(|e| e.into())
}
