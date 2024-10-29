mod action;

pub use action::TxnAction;
use chrono::NaiveDate;
use sea_query::enum_def;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[enum_def]
pub struct Transaction {
    #[serde(default)]
    id: Uuid,
    date: NaiveDate,
    account: String,
    action: TxnAction,
}
