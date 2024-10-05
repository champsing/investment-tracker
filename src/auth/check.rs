use crate::{auth::verify, error::Result};
use actix_web::{get, web::Json, HttpResponse, Responder};
use serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Deserialize)]
struct Request {
    token: String,
}

#[get("/api/auth/check")]
pub async fn handler(request: Json<Request>) -> Result<impl Responder> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    match verify(&request.token, now) {
        Some(_) => Ok(HttpResponse::Ok().finish()),
        None => Ok(HttpResponse::Forbidden().finish()),
    }
}
