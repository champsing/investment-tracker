use crate::database;
use crate::error::ServerError;
use crate::user::authenticate;
use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Deserialize)]
struct Request {
    token: String,
    username: Option<String>,
    password: Option<String>,
}

#[post("/api/user/update")]
pub async fn handler(
    request: web::Json<Request>,
) -> Result<impl Responder, ServerError> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

    // permission check
    let id = match authenticate(&request.token, now) {
        None => return Ok(HttpResponse::Forbidden().finish()),
        Some(i) => i,
    };
    let mut user = match database::users::select(Some(id), None)? {
        None => return Ok(HttpResponse::BadRequest().finish()),
        Some(u) => u,
    };

    // update values
    if let Some(username) = request.username.clone() {
        user.username = username
    }
    if let Some(password) = request.password.clone() {
        user.password = Sha256::digest(password).to_vec()
    }

    database::users::update(user)?;
    Ok(HttpResponse::Ok().finish())
}
