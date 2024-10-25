use crate::database;
use crate::error::ServerError;
use crate::user::authenticate;
use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Deserialize)]
struct Request {
    token: String,
}

#[post("/api/investment/account/fetch")]
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

    let accounts = database::accounts::select_by_user(user_id)?;
    Ok(HttpResponse::Ok().json(accounts))
}
