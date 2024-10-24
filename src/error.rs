use actix_web::ResponseError;
use derive_more::{Display, From};

#[derive(Debug, Display, From)]
pub enum ServerError {
    Time(std::time::SystemTimeError),
    Io(std::io::Error),
    Sqlite(rusqlite::Error),
    Json(serde_json::Error),
    Jwt(jwt::Error),
    #[from(skip)]
    Internal(String),
}

impl ResponseError for ServerError {}
