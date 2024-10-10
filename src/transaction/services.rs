pub mod search {
    use crate::{auth::verify, error::Result, transaction::database};
    use actix_web::{post, web, HttpResponse, Responder};
    use chrono::NaiveDate;
    use serde::Deserialize;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[derive(Debug, Deserialize)]
    struct Request {
        token: String,
        min_date: NaiveDate,
        max_date: NaiveDate,
    }

    #[post("/api/transaction/search")]
    pub async fn handler(request: web::Json<Request>) -> Result<impl Responder> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        match verify(&request.token, now) {
            Some(_) => {
                let transactions = database::search(&request.min_date, &request.max_date)?;
                Ok(HttpResponse::Ok().json(transactions))
            }
            _ => Ok(HttpResponse::Forbidden().finish()),
        }
    }
}

pub mod upsert {
    use crate::auth::{verify, UserGroup};
    use crate::error::Result;
    use crate::transaction::{database, Transaction};
    use actix_web::{post, web, HttpResponse, Responder};
    use serde::Deserialize;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[derive(Debug, Deserialize)]
    struct Request {
        token: String,
        transaction: Transaction,
    }

    #[post("/api/transaction/upsert")]
    pub async fn handler(request: web::Json<Request>) -> Result<impl Responder> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        match verify(&request.token, now) {
            Some(UserGroup::Editor) => {
                database::upsert(&request.transaction)?;
                Ok(HttpResponse::Ok().finish())
            }
            _ => Ok(HttpResponse::Forbidden().finish()),
        }
    }
}

pub mod delete {
    use crate::auth::{verify, UserGroup};
    use crate::error::Result;
    use crate::transaction::database;
    use actix_web::{post, web, HttpResponse, Responder};
    use serde::Deserialize;
    use std::time::{SystemTime, UNIX_EPOCH};
    use uuid::Uuid;

    #[derive(Debug, Deserialize)]
    struct Request {
        token: String,
        id: Uuid,
    }

    #[post("/api/transaction/delete")]
    pub async fn handler(request: web::Json<Request>) -> Result<impl Responder> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        match verify(&request.token, now) {
            Some(UserGroup::Editor) => {
                database::delete(&request.id)?;
                Ok(HttpResponse::Ok().finish())
            }
            _ => Ok(HttpResponse::Forbidden().finish()),
        }
    }
}
