mod migration;

use crate::error::ServerError;
use std::fs;

const DATABASE: &str = "data/sqlite.db";

pub fn init() -> Result<(), ServerError> {
    fs::create_dir_all("data/")?;
    migration::run_migration()?;
    Ok(())
}
