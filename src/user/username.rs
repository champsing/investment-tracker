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

#[post("/api/user/username")]
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

    let user = database::users::select(Some(user_id), None)?;
    if user.is_none() {
        return Ok(HttpResponse::BadRequest().finish());
    }
    let user = user.unwrap();
    Ok(HttpResponse::Ok().body(user.username))
}
