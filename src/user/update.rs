use crate::database;
use crate::database::users::User;
use crate::error::ServerError;
use crate::user::authenticate;
use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Deserialize)]
struct Request {
    token: String,
    user: User,
}

#[post("/api/user/update")]
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
    if user_id != request.user.id {
        return Ok(HttpResponse::Forbidden().finish());
    }

    database::users::update(request.user.clone())?;
    Ok(HttpResponse::Ok().finish())
}
