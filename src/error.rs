use actix_web::ResponseError;
use derive_more::{Display, From};

#[derive(Debug, Display, From)]
pub enum ServerError {
    Polodb(polodb_core::Error),
    Jwt(jwt::Error),
    SystemTime(std::time::SystemTimeError),
    Io(std::io::Error),
}

impl ResponseError for ServerError {}

pub type Result<T> = std::result::Result<T, ServerError>;
