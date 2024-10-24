use crate::database;
use crate::error::ServerError;
use crate::user::authenticate;
use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct Request {
    token: String,
    user_id: Uuid,
}

#[post("/api/user/delete")]
pub async fn handler(
    request: web::Json<Request>,
) -> Result<impl Responder, ServerError> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

    let user_id = authenticate(&request.token, now);
    if user_id.is_none() {
        return Ok(HttpResponse::Forbidden().finish());
    }

    let user_id = user_id.unwrap();
    if user_id != request.user_id {
        return Ok(HttpResponse::Forbidden().finish());
    }

    database::users::delete(request.user_id)?;
    Ok(HttpResponse::Ok().finish())
}
