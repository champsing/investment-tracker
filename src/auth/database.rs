use super::UserGroup;
use crate::constant::path;
use crate::error::Result;
use const_format::formatcp as const_format;
use polodb_core::{bson::doc, CollectionT, Database};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

const DATABASE: &str = const_format!("{}/credential.db", path::DATA);

#[derive(Debug, Serialize, Deserialize)]
struct User {
    username: String,
    password: Vec<u8>,
    group: UserGroup,
}

pub fn login(username: &str, password: &str) -> Result<Option<UserGroup>> {
    let database = Database::open_path(DATABASE)?;
    let collection = database.collection::<User>("users");

    let count = collection.count_documents()?;
    if count == 0 && username == "admin" && password == "admin" {
        return Ok(Some(UserGroup::Editor));
    }

    let user = collection
        .find(doc! {
            "username": username
        })
        .run()?
        .next();
    if let Some(user) = user {
        let user = user?;

        let password: Vec<_> = Sha256::digest(password).into_iter().collect();
        if password == user.password {
            return Ok(Some(user.group));
        }
    }

    return Ok(None);
}

pub fn contains(username: String) -> Result<bool> {
    let database = Database::open_path(DATABASE)?;
    let collection = database.collection::<User>("users");

    let count = collection
        .find(doc! {
            "username": username
        })
        .run()?
        .count();

    Ok(count != 0)
}
