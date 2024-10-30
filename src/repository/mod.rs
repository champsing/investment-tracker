mod currency;

use crate::database::asset::Asset;
use crate::error::ServerError;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssetKind {
    Currency,
    Stock,
    ETF,
    Crypto,
    Unknown,
}

pub struct AssetInfo {
    id: Asset,
    name: String,
    kind: AssetKind,
}

pub trait IRepository {
    async fn search(
        &self,
        user_id: Uuid,
        prefix: String,
        kinds: Vec<AssetKind>,
    ) -> Result<Vec<AssetInfo>, ServerError>;
}

pub struct Repository;

impl IRepository for Repository {
    async fn search(
        &self,
        user_id: Uuid,
        prefix: String,
        kinds: Vec<AssetKind>,
    ) -> Result<Vec<AssetInfo>, ServerError> {
        let repo = currency::Repository;
        match repo.search(user_id, prefix, kinds).await {
            Ok(result) => return Ok(result),
            _ => (),
        }

        todo!()
    }
}
