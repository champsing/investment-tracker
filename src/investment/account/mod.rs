mod database;
pub mod portfolio;
mod services;

use super::INVESTMENT_DATABASE;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AccountType {
    NRA,
    TFSA,
    FHSA,
    RRSP,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Account {
    id: String,
    alias: String,
    tag: AccountType,
}

pub use database::init;
pub use services::delete::handler as delete;
pub use services::query::handler as query;
pub use services::upsert::handler as upsert;
