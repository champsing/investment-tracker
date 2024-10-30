use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Asset {
    // stock or ETF, anything tradable through stock exchanges
    STOCK { exchange: String, ticker: String },
    // fiat currency backed by a sovereign state government.
    CURRENCY(String),
    // crypto currency
    CRYPTO(String),
    // anything else
    UNKNOWN(String),
}

impl From<String> for Asset {
    fn from(value: String) -> Self {
        let mut iter = value.split(":");
        let split: [&str; 2] = [(); 2].map(|_| iter.next().unwrap());
        let kind = split[0];
        let symbol = split[1];

        match kind {
            "CURRENCY" => Self::CURRENCY(String::from(symbol)),
            "CRYPTO" => Self::CRYPTO(String::from(symbol)),
            "UNKNOWN" => Self::UNKNOWN(String::from(symbol)),
            s => Self::STOCK {
                exchange: String::from(&s[1..s.len()]),
                ticker: String::from(symbol),
            },
        }
    }
}

impl From<Asset> for String {
    fn from(value: Asset) -> Self {
        match value {
            Asset::STOCK { exchange, ticker } => {
                format!("X{}:{}", exchange, ticker)
            }
            Asset::CURRENCY(symbol) => format!("CURRENCY:{}", symbol),
            Asset::CRYPTO(symbol) => format!("CRYPTO:{}", symbol),
            Asset::UNKNOWN(symbol) => format!("UNKNOWN:{}", symbol),
        }
    }
}

impl Serialize for Asset {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value: String = self.clone().into();
        value.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Asset {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Ok(Asset::from(value))
    }
}
