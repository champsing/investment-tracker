use actix_web::ResponseError;
use derive_more::{Display, From};

#[derive(Debug, Display, From)]
pub enum ServerError {
    SystemTime(std::time::SystemTimeError),
    Io(std::io::Error),
    Polodb(polodb_core::Error),
    Bson(polodb_core::bson::ser::Error),
    Jwt(jwt::Error),
}

impl ResponseError for ServerError {}

pub type Result<T> = std::result::Result<T, ServerError>;
