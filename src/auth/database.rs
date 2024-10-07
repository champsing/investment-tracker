use super::UserGroup;
use crate::constant::path;
use crate::error::Result;
use const_format::formatcp as const_format;
use polodb_core::{bson, CollectionT, Database, IndexModel, IndexOptions};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

const DATABASE: &str = const_format!("{}/credential.db", path::DATA);

#[derive(Debug, Serialize, Deserialize)]
struct User {
    username: String,
    password: Vec<u8>,
    group: UserGroup,
}

pub fn init() -> Result<()> {
    let database = Database::open_path(DATABASE)?;
    let collection = database.collection::<User>("users");
    collection.create_index(IndexModel {
        keys: bson::doc! {
            "username": 1,
        },
        options: Some(IndexOptions {
            name: Some(String::from("username_1")),
            unique: Some(true),
        }),
    })?;

    Ok(())
}

pub fn login(username: &str, password: &str) -> Result<Option<UserGroup>> {
    let database = Database::open_path(DATABASE)?;
    let collection = database.collection::<User>("users");

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

pub fn all() -> Result<Vec<(String, UserGroup)>> {
    let database = Database::open_path(DATABASE)?;
    let collection = database.collection::<User>("users");

    let users: std::result::Result<Vec<_>, _> = collection
        .find(bson::doc! {})
        .run()?
        .map(|r| r.map(|u| (u.username, u.group)))
        .collect();

    Ok(users?)
}

pub fn insert(username: &str, password: &str, group: UserGroup) -> Result<()> {
    let database = Database::open_path(DATABASE)?;
    let transition = database.start_transaction()?;
    let collection = transition.collection::<User>("users");

    let count = collection
        .find(bson::doc! {
            "username": username
        })
        .run()?
        .count();

    let password = Sha256::digest(password).to_vec();
    if count == 0 {
        collection.insert_one(User {
            username: String::from(username),
            password,
            group,
        })?;
    } else {
        collection.update_one(
            bson::doc! {
                "username": username
            },
            bson::doc! {
                "$set": bson::doc! {
                    "password": bson::to_bson(&password)?,
                    "group": bson::to_bson(&group)?
                },
            },
        )?;
    }
    transition.commit()?;

    Ok(())
}

pub fn delete(username: &str) -> Result<()> {
    let database = Database::open_path(DATABASE)?;
    let collection = database.collection::<User>("users");

    collection.delete_one(bson::doc! {
        "username": String::from(username)
    })?;

    Ok(())
}
