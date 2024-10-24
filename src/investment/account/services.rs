pub mod insert {

}

pub mod upsert {
    use super::super::{database, Account};
    use crate::{
        auth::{verify, UserGroup},
        error::Result,
    };
    use actix_web::{post, web, HttpResponse, Responder};
    use serde::Deserialize;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[derive(Debug, Deserialize)]
    struct Request {
        token: String,
        account: Account,
    }

    #[post("/api/investment/account/upsert")]
    pub async fn handler(request: web::Json<Request>) -> Result<impl Responder> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        match verify(&request.token, now) {
            Some(UserGroup::Editor) => {
                if request.account.id.len() > 0 && request.account.alias.len() > 0 {
                    database::upsert(&request.account)?;
                    Ok(HttpResponse::Ok().finish())
                } else {
                    Ok(HttpResponse::BadRequest().finish())
                }
            }
            _ => Ok(HttpResponse::Forbidden().finish()),
        }
    }
}

pub mod delete {
    use super::super::database;
    use crate::{
        auth::{verify, UserGroup},
        error::Result,
    };
    use actix_web::{post, web, HttpResponse, Responder};
    use serde::Deserialize;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[derive(Debug, Deserialize)]
    struct Request {
        token: String,
        id: String,
    }

    #[post("/api/investment/account/delete")]
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
