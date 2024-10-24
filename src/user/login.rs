use super::PRIVATE_KEY;
use crate::database;
use crate::error::ServerError;
use crate::user::Claims;
use actix_web::{post, web, HttpResponse, Responder};
use jwt::SignWithKey;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Deserialize)]
struct Request {
    username: String,
    password: String,
}

#[post("/api/user/login")]
pub async fn handler(
    request: web::Json<Request>,
) -> Result<impl Responder, ServerError> {
    let user = database::users::select(None, Some(request.username.clone()))?;

    if Sha256::digest(request.password.clone()).to_vec() != user.password {
        return Ok(HttpResponse::Forbidden().finish());
    }

    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let claims = Claims {
        iss: user.id,
        iat: now,
        exp: now + 3600,
    };
    let token = claims.sign_with_key(&*PRIVATE_KEY)?;
    Ok(HttpResponse::Ok().body(token))
}
