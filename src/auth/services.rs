use super::{database, UserGroup};
use hmac::{Hmac, Mac};
use jwt::{Header, Token, VerifyWithKey};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::sync::LazyLock;

static PRIVATE_KEY: LazyLock<Hmac<Sha256>> = LazyLock::new(|| {
    let mut rng = rand::thread_rng();
    let mut bytes = [0_u8; 32];
    rng.fill_bytes(&mut bytes);
    Hmac::new_from_slice(&bytes).expect("fail to generate HMAC key.")
});

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: UserGroup,
    iat: u64,
    exp: u64,
}

pub fn verify(token: &str, now: u64) -> Option<UserGroup> {
    match token.verify_with_key(&*PRIVATE_KEY).ok() {
        Some(token) => {
            let token: Token<Header, Claims, _> = token;
            let claims: &Claims = token.claims();
            if claims.exp > now {
                Some(claims.iss)
            } else {
                None
            }
        }
        _ => None,
    }
}

pub mod login {
    use super::{database, Claims, PRIVATE_KEY};
    use crate::error::Result;
    use actix_web::{post, web, HttpResponse, Responder};
    use jwt::SignWithKey;
    use serde::Deserialize;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[derive(Debug, Deserialize)]
    struct Request {
        username: String,
        password: String,
    }

    #[post("/api/auth/login")]
    pub async fn handler(request: web::Json<Request>) -> Result<impl Responder> {
        match database::login(&request.username, &request.password)? {
            Some(user_group) => {
                let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
                let claims = Claims {
                    iss: user_group,
                    iat: now,
                    exp: now + 3600,
                };
                let token = claims.sign_with_key(&*PRIVATE_KEY)?;
                Ok(HttpResponse::Ok().body(token))
            }
            _ => Ok(HttpResponse::Forbidden().finish()),
        }
    }
}

pub mod refresh {
    use super::{verify, Claims, PRIVATE_KEY};
    use crate::error::Result;
    use actix_web::{post, web, HttpResponse, Responder};
    use jwt::SignWithKey;
    use serde::Deserialize;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[derive(Debug, Deserialize)]
    struct Request {
        token: String,
    }

    #[post("/api/auth/refresh")]
    pub async fn handler(request: web::Json<Request>) -> Result<impl Responder> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        match verify(&request.token, now) {
            Some(user_group) => {
                let claims = Claims {
                    iss: user_group,
                    iat: now,
                    exp: now + 3600,
                };
                let token = claims.sign_with_key(&*PRIVATE_KEY)?;
                Ok(HttpResponse::Ok().body(token))
            }
            _ => Ok(HttpResponse::Forbidden().finish()),
        }
    }
}

pub mod all_users {
    use super::verify;
    use crate::{
        auth::{database, UserGroup},
        error::Result,
    };
    use actix_web::{post, web, HttpResponse, Responder};
    use serde::{Deserialize, Serialize};
    use std::time::{SystemTime, UNIX_EPOCH};

    #[derive(Debug, Deserialize)]
    struct Request {
        token: String,
    }

    #[derive(Debug, Serialize)]
    struct User {
        username: String,
        group: UserGroup,
    }

    #[post("/api/auth/all_users")]
    pub async fn handler(request: web::Json<Request>) -> Result<impl Responder> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        match verify(&request.token, now) {
            Some(UserGroup::Editor) => {
                let users: Vec<_> = database::all()?
                    .into_iter()
                    .map(|u| User {
                        username: u.0,
                        group: u.1,
                    })
                    .collect();
                Ok(HttpResponse::Ok().json(users))
            }
            _ => Ok(HttpResponse::Forbidden().finish()),
        }
    }
}

pub mod insert {
    use super::{database, verify, UserGroup};
    use crate::error::Result;
    use actix_web::{put, web, HttpResponse, Responder};
    use serde::Deserialize;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[derive(Debug, Deserialize)]
    struct Request {
        token: String,
        username: String,
        password: String,
        group: UserGroup,
    }

    #[put("/api/auth/insert")]
    pub async fn handler(request: web::Json<Request>) -> Result<impl Responder> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        match verify(&request.token, now) {
            Some(UserGroup::Editor) => {
                database::insert(&request.username, &request.password, request.group)?;

                Ok(HttpResponse::Ok().finish())
            }
            _ => Ok(HttpResponse::Forbidden().finish()),
        }
    }
}

pub mod delete {
    use super::{database, verify, UserGroup};
    use crate::error::Result;
    use actix_web::{delete, web, HttpResponse, Responder};
    use serde::Deserialize;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[derive(Debug, Deserialize)]
    struct Request {
        token: String,
        username: String,
    }

    #[delete("/api/auth/delete")]
    pub async fn handler(request: web::Json<Request>) -> Result<impl Responder> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        match verify(&request.token, now) {
            Some(UserGroup::Editor) => {
                database::delete(&request.username)?;

                Ok(HttpResponse::Ok().finish())
            }
            _ => Ok(HttpResponse::Forbidden().finish()),
        }
    }
}
