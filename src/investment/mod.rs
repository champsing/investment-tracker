pub mod account;
mod transaction;

use crate::{constant::path, error::Result};
use const_format::formatcp as const_format;
use rust_decimal::Decimal;

const INVESTMENT_DATABASE: &str = const_format!("{}/investment.db", path::DATA);

trait IPortfolio {
    fn update(&mut self, symbol: &str, count: &Decimal);
}
