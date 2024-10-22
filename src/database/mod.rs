use configs::VERSION;
use log::info;

use crate::error::ServerError;

pub mod configs;

const DATABASE: &str = "data/sqlite.db";
const SCHEMA_VERSION: u64 = 1;

pub fn init() -> Result<(), ServerError> {
    configs::init()?;

    match configs::get(configs::VERSION)? {
        None => {
            // TODO: normal initialization
        }
        Some(version) => {
            let version = version.parse::<u64>()?;
            info!("database is currently at version {}", version);
            // TODO: migration code here
        }
    }

    configs::set(VERSION, format!("{}", SCHEMA_VERSION).as_str());

    Ok(())
}
