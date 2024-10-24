pub mod accounts;
pub mod users;
mod migration;

use crate::error::ServerError;
use std::fs;

const DATABASE: &str = "data/sqlite.db";

pub fn init() -> Result<(), ServerError> {
    use sea_query::Iden;
    println!("{}", accounts::AccountIden::Table.to_string());
    println!("{}", accounts::AccountIden::Name.to_string());
    println!("{}", accounts::AccountIden::Kind.to_string());

    fs::create_dir_all("data/")?;
    migration::run_migration()?;
    Ok(())
}
