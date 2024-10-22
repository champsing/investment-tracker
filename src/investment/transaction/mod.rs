mod actions;
mod database;

use super::IPortfolio;
use actions::Action;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    #[serde(default)]
    id: Uuid,
    date: NaiveDate,
    account: String,
    action: Action,
    fee: (String, Decimal),
}
