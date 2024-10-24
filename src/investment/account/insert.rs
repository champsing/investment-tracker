use crate::database;
use crate::database::accounts::Account;
use crate::error::ServerError;
use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;

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
    // TODO: verify the token.

    let mut account = request.account.clone();
    if !account.id.is_nil() {
        return Ok(HttpResponse::BadRequest().body("id should be nil"));
    } else if account.acct == "" {
        return Ok(HttpResponse::BadRequest().body("acct should not be empty"));
    } else if account.name == "" {
        // by default we will use acct for name
        account.name = account.acct.clone();
    }
    database::accounts::insert(account)?;

    Ok(HttpResponse::Ok().finish())
}
