use super::DATABASE;
use crate::error::ServerError;
use rusqlite::{params, Connection, OptionalExtension, Result};

pub const VERSION: &str = "VERSION";

pub(super) fn init() -> Result<(), ServerError> {
    let connection = Connection::open(DATABASE)?;
    connection.execute(include_str!("create.sql"), ())?;

    Ok(())
}

pub fn get(key: &str) -> Result<Option<String>, ServerError> {
    let connection = Connection::open(DATABASE)?;
    let mut statement = connection.prepare(include_str!("get.sql"))?;
    let result = statement
        .query_row(params![key], |row| row.get(0))
        .optional()?;
    Ok(result)
}

pub fn set(key: &str, value: &str) -> Result<(), ServerError> {
    let connection = Connection::open(DATABASE)?;
    connection.execute(include_str!("set.sql"), params![key, value])?;

    Ok(())
}
