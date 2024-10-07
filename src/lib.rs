pub mod auth;
pub mod constant;
mod error;

use std::fs;
use actix_files::NamedFile;
use actix_web::Responder;
use const_format::formatcp as const_format;
pub use error::Result;

pub fn init() -> Result<()> {
    fs::create_dir_all(constant::path::CACHE)?;
    fs::create_dir_all(constant::path::DATA)?;
    fs::create_dir_all(constant::path::WEB)?;
    
    auth::init()?;

    Ok(())
}

pub async fn index() -> Result<impl Responder> {
    const INDEX: &str = const_format!("{}/index.html", constant::path::WEB);
    NamedFile::open_async(INDEX).await.map_err(|e| e.into())
}
