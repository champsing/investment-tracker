use crate::database;
use crate::database::accounts::Account;
use crate::error::ServerError;
use crate::user::authenticate;
use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Deserialize)]
struct Request {
    #[allow(unused)]
    token: String,
    account: Account,
}

#[post("/api/investment/account/insert")]
pub async fn handler(
    request: web::Json<Request>,
) -> Result<impl Responder, ServerError> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

    // permission check
    let user_id = authenticate(&request.token, now);
    if user_id.is_none() {
        return Ok(HttpResponse::Forbidden().finish());
    }
    let user_id = user_id.unwrap();
    if user_id != request.account.owner {
        return Ok(HttpResponse::Forbidden().finish());
    }

    // input check
    if !request.account.id.is_nil() {
        return Ok(HttpResponse::BadRequest().body("id should be nil"));
    } else if request.account.name.len() < 6 {
        return Ok(HttpResponse::BadRequest().body("account name too short"));
    }

    database::accounts::insert(request.account.clone())?;
    Ok(HttpResponse::Ok().finish())
}
