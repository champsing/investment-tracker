use actix_web::ResponseError;
use derive_more::{Display, From};
use std::time::SystemTimeError;

#[derive(Debug, Display, From)]
pub enum ServerError {
    Polodb(polodb_core::Error),
    Jwt(jwt::Error),
    SystemTime(SystemTimeError),
}

impl ResponseError for ServerError {}

pub type Result<T> = std::result::Result<T, ServerError>;
