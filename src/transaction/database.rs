use super::action::Transaction;
use crate::{constant::path, error::Result};
use chrono::NaiveDate;
use const_format::formatcp as const_format;
use polodb_core::{bson, options::UpdateOptions, CollectionT, Database, IndexModel, IndexOptions};
use uuid::Uuid;

const DATABASE: &str = const_format!("{}/account.db", path::DATA);

pub fn init() -> Result<()> {
    let database = Database::open_path(DATABASE)?;
    let collection = database.collection::<Transaction>("transactions");
    collection.create_index(IndexModel {
        keys: bson::doc! {
            "id": 1,
        },
        options: Some(IndexOptions {
            name: Some(String::from("id_1")),
            unique: Some(false),
        }),
    })?;

    collection.create_index(IndexModel {
        keys: bson::doc! {
            "date": 1,
        },
        options: Some(IndexOptions {
            name: Some(String::from("date_1")),
            unique: Some(false),
        }),
    })?;

    Ok(())
}

pub fn search(min_date: &NaiveDate, max_date: &NaiveDate) -> Result<Vec<Transaction>> {
    let database = Database::open_path(DATABASE)?;
    let collection = database.collection::<Transaction>("transactions");

    let result: std::result::Result<Vec<_>, _> = collection
        .find(bson::doc! {
            "date": {
                "$gte": bson::to_bson(min_date)?,
                "$lte": bson::to_bson(max_date)?
            }
        })
        .run()?
        .collect();
    Ok(result?)
}

pub fn upsert(transaction: &Transaction) -> Result<()> {
    assert!(!transaction.id.is_nil());

    let database = Database::open_path(DATABASE)?;
    let collection = database.collection::<Transaction>("transactions");

    collection.update_one_with_options(
        bson::doc! {
            "id": bson::to_bson(&transaction.id)?
        },
        bson::doc! {
            "$set": bson::doc! {
                "date": bson::to_bson(&transaction.date)?,
                "account": bson::to_bson(&transaction.account)?,
                "action": bson::to_bson(&transaction.action)?,
            },
        },
        UpdateOptions { upsert: Some(true) },
    )?;

    Ok(())
}

pub fn delete(id: &Uuid) -> Result<()> {
    assert!(!id.is_nil());

    let database = Database::open_path(DATABASE)?;
    let collection = database.collection::<Transaction>("transactions");

    collection.delete_one(bson::doc! {
        "id": bson::to_bson(id)?
    })?;

    Ok(())
}
