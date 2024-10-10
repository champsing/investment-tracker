use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Portfolio(HashMap<String, i64>);

impl Portfolio {
    pub fn update(&mut self, symbol: &str, count: i64) {
        let old_value = self.0.remove(symbol).unwrap_or(0);
        let new_value = old_value + count;
        if new_value != 0 {
            self.0.insert(String::from(symbol), new_value);
        }
    }
}
