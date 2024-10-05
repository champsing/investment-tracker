use super::{database, UserGroup};
use crate::error::Result;
use actix_web::{post, web::Json, HttpResponse, Responder};
use hmac::{Hmac, Mac};
use jwt::{Header, SignWithKey, Token, VerifyWithKey};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::sync::LazyLock;
use std::time::{SystemTime, UNIX_EPOCH};

static PRIVATE_KEY: LazyLock<Hmac<Sha256>> = LazyLock::new(|| {
    let mut rng = rand::thread_rng();
    let mut bytes = [0_u8; 32];
    rng.fill_bytes(&mut bytes);
    Hmac::new_from_slice(&bytes).expect("fail to generate HMAC key.")
});

#[derive(Debug, Deserialize)]
struct Request {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: UserGroup,
    iat: u64,
    exp: u64,
}

#[post("/api/auth/login")]
pub async fn handler(request: Json<Request>) -> Result<impl Responder> {
    let user_group = database::login(&request.username, &request.password)?;

    if let Some(user_group) = user_group {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let claims = Claims {
            iss: user_group,
            iat: now,
            exp: now + 3600,
        };
        let token = claims.sign_with_key(&*PRIVATE_KEY)?;
        Ok(HttpResponse::Ok().body(token))
    } else {
        Ok(HttpResponse::Forbidden().finish())
    }
}

pub fn verify(token: &str, now: u64) -> Option<UserGroup> {
    match token.verify_with_key(&*PRIVATE_KEY).ok() {
        None => None,
        Some(token) => {
            let token: Token<Header, Claims, _> = token;
            let claims: &Claims = token.claims();
            if claims.exp < now {
                Some(claims.iss)
            } else {
                None
            }
        }
    }
}
