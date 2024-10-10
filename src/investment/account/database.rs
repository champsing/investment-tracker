use super::{Account, INVESTMENT_DATABASE};
use crate::error::Result;
use polodb_core::{bson, options::UpdateOptions, CollectionT, Database, IndexModel};

const ACCOUNT_COLLECTION: &str = "accounts";

pub fn init() -> Result<()> {
    let database = Database::open_path(INVESTMENT_DATABASE)?;
    let collection = database.collection::<Account>(ACCOUNT_COLLECTION);
    collection.create_index(IndexModel {
        keys: bson::doc! {
            "id": 1,
        },
        options: None,
    })?;

    collection.create_index(IndexModel {
        keys: bson::doc! {
            "tag": 1,
        },
        options: None,
    })?;

    Ok(())
}

pub fn query() -> Result<Vec<Account>> {
    let database = Database::open_path(INVESTMENT_DATABASE)?;
    let collection = database.collection::<Account>(ACCOUNT_COLLECTION);

    let result: std::result::Result<Vec<_>, _> = collection.find(bson::doc! {}).run()?.collect();
    Ok(result?)
}

pub fn upsert(account: &Account) -> Result<()> {
    let database = Database::open_path(INVESTMENT_DATABASE)?;
    let collection = database.collection::<Account>(ACCOUNT_COLLECTION);

    collection.update_one_with_options(
        bson::doc! {
            "id": bson::to_bson(&account.id)?
        },
        bson::doc! {
            "$set": bson::doc! {
                "alias": bson::to_bson(&account.alias)?,
                "tag": bson::to_bson(&account.tag)?,
            },
        },
        UpdateOptions { upsert: Some(true) },
    )?;

    Ok(())
}

pub fn delete(id: &String) -> Result<()> {
    let database = Database::open_path(INVESTMENT_DATABASE)?;
    let collection = database.collection::<Account>(ACCOUNT_COLLECTION);

    collection.delete_one(bson::doc! {
        "id": bson::to_bson(id)?
    })?;

    Ok(())
}
