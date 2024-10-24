use super::DATABASE;
use crate::error::ServerError;
use core::str;
use rusqlite::types::{FromSql, FromSqlError, ValueRef};
use rusqlite::Connection;
use sea_query::{enum_def, Query, SqliteQueryBuilder};
use sea_query_rusqlite::RusqliteBinder;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[enum_def]
pub struct Account {
    #[serde(default)]
    pub id: Uuid,
    pub acct: String,
    pub name: String,
    pub kind: AccountKind,
}

impl PartialEq for Account {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Account {}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum AccountKind {
    NRA,
    TFSA,
    RRSP,
    FHSA,
}

impl From<AccountKind> for sea_query::value::Value {
    fn from(value: AccountKind) -> Self {
        match value {
            AccountKind::NRA => "NRA".into(),
            AccountKind::TFSA => "TFSA".into(),
            AccountKind::RRSP => "RRSP".into(),
            AccountKind::FHSA => "FHSA".into(),
        }
    }
}

impl FromSql for AccountKind {
    fn column_result(value: ValueRef<'_>) -> Result<Self, FromSqlError> {
        if let ValueRef::Text(text) = value {
            if let Ok(s) = str::from_utf8(text) {
                if s == "NRA" {
                    return Ok(Self::NRA);
                } else if s == "TFSA" {
                    return Ok(Self::TFSA);
                } else if s == "RRSP" {
                    return Ok(Self::RRSP);
                } else if s == "FHSA" {
                    return Ok(Self::FHSA);
                }
            }
        }

        Err(FromSqlError::InvalidType)
    }
}

pub fn insert(account: Account) -> Result<(), ServerError> {
    assert!(account.id.is_nil());

    let (query, values) = Query::insert()
        .into_table(AccountIden::Table)
        .columns([
            AccountIden::Id,
            AccountIden::Acct,
            AccountIden::Name,
            AccountIden::Kind,
        ])
        .values([
            Uuid::new_v4().into(),
            account.acct.into(),
            account.name.into(),
            account.kind.into(),
        ])?
        .build_rusqlite(SqliteQueryBuilder);

    let connection = Connection::open(DATABASE)?;
    connection.execute(&query, &*values.as_params())?;
    Ok(())
}

// pub fn update(account: Account) -> Result<(), ServerError> {
//     let connection = Connection::open(DATABASE)?;
//     connection.execute(
//         include_str!("update.sql"),
//         (account.id, account.acct, account.name, account.typ_),
//     )?;

//     Ok(())
// }

// pub fn delete(id: Uuid) -> Result<(), ServerError> {
//     let connection = Connection::open(DATABASE)?;
//     connection.execute("", ())?;
//     Ok(())
// }
