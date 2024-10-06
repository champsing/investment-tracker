pub mod auth;
pub mod constant;
mod error;

use actix_files::NamedFile;
use actix_web::Responder;
use const_format::formatcp as const_format;
use error::Result;

pub async fn index() -> Result<impl Responder> {
    const INDEX: &str = const_format!("{}/index.html", constant::path::WEB);
    NamedFile::open_async(INDEX).await.map_err(|e| e.into())
}
