use super::{UserGroup, CREDENTIAL_DATABASE};
use crate::error::Result;
use polodb_core::{bson, options::UpdateOptions, CollectionT, Database, IndexModel, IndexOptions};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

const USER_COLLECTION: &str = "users";

#[derive(Debug, Serialize, Deserialize)]
struct User {
    username: String,
    password: Vec<u8>,
    group: UserGroup,
}

pub fn init() -> Result<()> {
    let database = Database::open_path(CREDENTIAL_DATABASE)?;
    let collection = database.collection::<User>(USER_COLLECTION);
    collection.create_index(IndexModel {
        keys: bson::doc! {
            "username": 1,
        },
        options: Some(IndexOptions {
            name: Some(String::from("username_1")),
            unique: Some(false),
        }),
    })?;

    Ok(())
}

pub fn login(username: &str, password: &str) -> Result<Option<UserGroup>> {
    let database = Database::open_path(CREDENTIAL_DATABASE)?;
    let collection = database.collection::<User>(USER_COLLECTION);

    let count = collection
        .find(bson::doc! {
            "group": bson::to_bson(&UserGroup::Editor)?
        })
        .run()?
        .count();
    if count == 0 && username == "admin" && password == "admin" {
        return Ok(Some(UserGroup::Editor));
    }

    let user = collection
        .find(bson::doc! {
            "username": username
        })
        .run()?
        .next();
    if let Some(user) = user {
        let user = user?;

        let password = Sha256::digest(password).to_vec();
        if password == user.password {
            return Ok(Some(user.group));
        }
    }

    return Ok(None);
}

pub fn query() -> Result<Vec<(String, UserGroup)>> {
    let database = Database::open_path(CREDENTIAL_DATABASE)?;
    let collection = database.collection::<User>(USER_COLLECTION);

    let users: std::result::Result<Vec<_>, _> = collection
        .find(bson::doc! {})
        .run()?
        .map(|r| r.map(|u| (u.username, u.group)))
        .collect();

    Ok(users?)
}

pub fn upsert(username: &str, password: &str, group: UserGroup) -> Result<()> {
    let database = Database::open_path(CREDENTIAL_DATABASE)?;
    let collection = database.collection::<User>(USER_COLLECTION);

    let password = Sha256::digest(password).to_vec();
    collection.update_one_with_options(
        bson::doc! {
            "username": username
        },
        bson::doc! {
            "$set": bson::doc! {
                "password": bson::to_bson(&password)?,
                "group": bson::to_bson(&group)?
            },
        },
        UpdateOptions { upsert: Some(true) },
    )?;

    Ok(())
}

pub fn delete(username: &str) -> Result<()> {
    let database = Database::open_path(CREDENTIAL_DATABASE)?;
    let collection = database.collection::<User>(USER_COLLECTION);

    collection.delete_one(bson::doc! {
        "username": String::from(username)
    })?;

    Ok(())
}
