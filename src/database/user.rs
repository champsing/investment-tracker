use super::DATABASE;
use crate::error::ServerError;
use core::str;
use rusqlite::Connection;
use sea_query::{enum_def, Expr, Query, SqliteQueryBuilder};
use sea_query_rusqlite::RusqliteBinder;
use serde::{Deserialize, Serialize};
use std::hash::Hash;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[enum_def]
pub struct User {
    #[serde(default)]
    pub id: Uuid,
    pub username: String,
    pub password: Vec<u8>,
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for User {}

impl Hash for User {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

pub fn insert(user: User) -> Result<(), ServerError> {
    assert!(user.id.is_nil());

    let (query, values) = Query::insert()
        .into_table(UserIden::Table)
        .columns([UserIden::Id, UserIden::Username, UserIden::Password])
        .values([
            Uuid::new_v4().into(),
            user.username.into(),
            user.password.into(),
        ])?
        .build_rusqlite(SqliteQueryBuilder);

    let connection = Connection::open(DATABASE)?;
    connection.execute(&query, &*values.as_params())?;
    Ok(())
}

pub fn update(user: User) -> Result<(), ServerError> {
    let (query, values) = Query::update()
        .table(UserIden::Table)
        .values([
            (UserIden::Username, user.username.into()),
            (UserIden::Password, user.password.into()),
        ])
        .and_where(Expr::col(UserIden::Id).eq(user.id))
        .build_rusqlite(SqliteQueryBuilder);

    let connection = Connection::open(DATABASE)?;
    connection.execute(&query, &*values.as_params())?;
    Ok(())
}

pub fn delete(id: Uuid) -> Result<(), ServerError> {
    let (query, values) = Query::delete()
        .from_table(UserIden::Table)
        .and_where(Expr::col(UserIden::Id).eq(id))
        .build_rusqlite(SqliteQueryBuilder);

    let connection = Connection::open(DATABASE)?;
    connection.execute(&query, &*values.as_params())?;
    Ok(())
}

pub fn select(
    id: Option<Uuid>,
    username: Option<String>,
) -> Result<Option<User>, ServerError> {
    let (query, values) = Query::select()
        .columns([UserIden::Id, UserIden::Username, UserIden::Password])
        .from(UserIden::Table)
        .and_where_option(id.map(|x| Expr::col(UserIden::Id).eq(x)))
        .and_where_option(
            username
                .as_ref()
                .map(|x| Expr::col(UserIden::Username).eq(x)),
        )
        .build_rusqlite(SqliteQueryBuilder);

    let connection = Connection::open(DATABASE)?;
    let mut statement = connection.prepare(&query)?;
    let record: Option<Result<_, rusqlite::Error>> = statement
        .query_and_then(&*values.as_params(), |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                password: row.get(2)?,
            })
        })?
        .next();

    Ok(record.transpose()?)
}
