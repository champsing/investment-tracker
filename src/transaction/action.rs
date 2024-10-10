use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type Entry = (String, Decimal);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    #[serde(default)]
    pub id: Uuid,
    pub date: NaiveDate,
    pub account: String,
    pub action: Action,
    pub fee: Entry,
    pub comment: String,
}

pub trait IAction {
    fn commands(&self) -> Vec<Entry>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub(super) enum Action {
    Deposit(Deposit),
    Withdrawal(Withdrawal),
    Buy(Buy),
    Sell(Sell),
}

impl IAction for Action {
    fn commands(&self) -> Vec<Entry> {
        match self {
            Action::Deposit(x) => x.commands(),
            Action::Withdrawal(x) => x.commands(),
            Action::Buy(x) => x.commands(),
            Action::Sell(x) => x.commands(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct Deposit {
    pub(super) value: Entry,
}

impl IAction for Deposit {
    fn commands(&self) -> Vec<Entry> {
        let x = self.clone();
        vec![x.value]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct Withdrawal {
    pub(super) value: Entry,
}

impl IAction for Withdrawal {
    fn commands(&self) -> Vec<Entry> {
        let x = self.clone();
        vec![x.value]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct Buy {
    pub(super) stock: Entry,
    pub(super) cash: Entry,
}

impl IAction for Buy {
    fn commands(&self) -> Vec<Entry> {
        let x = self.clone();
        vec![x.stock, x.cash]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct Sell {
    pub(super) stock: Entry,
    pub(super) cash: Entry,
}

impl IAction for Sell {
    fn commands(&self) -> Vec<Entry> {
        let x = self.clone();
        vec![x.stock, x.cash]
    }
}
