use crate::database;
use crate::error::ServerError;
use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Request {
    username: String,
}

#[post("/api/user/username")]
pub async fn handler(
    request: web::Json<Request>,
) -> Result<impl Responder, ServerError> {
    let user = database::users::select(None, Some(request.username.clone()))?
        .is_some();
    let value = serde_json::to_string(&user)?;
    Ok(HttpResponse::Ok().body(value))
}
