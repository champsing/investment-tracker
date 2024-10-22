use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Action {
    Deposit(Deposit),
    Withdrawal(Withdrawal),
    Buy(Buy),
    Sell(Sell),
    Interest(Interest)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deposit {
    pub value: (String, Decimal),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Withdrawal {
    pub value: (String, Decimal),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interest {
    pub value: (String, Decimal),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Buy {
    pub stock: (String, Decimal),
    pub cash: (String, Decimal),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sell {
    pub stock: (String, Decimal),
    pub cash: (String, Decimal),
}

