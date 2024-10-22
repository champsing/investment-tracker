use actix_web::ResponseError;
use derive_more::{Display, From};

#[derive(Debug, Display, From)]
pub enum ServerError {
    Time(std::time::SystemTimeError),
    Io(std::io::Error),
    ParseInt(core::num::ParseIntError),
    Sqlite(rusqlite::Error),
    Polodb(polodb_core::Error),
    Bson(polodb_core::bson::ser::Error),
    Jwt(jwt::Error),
}

impl ResponseError for ServerError {}
