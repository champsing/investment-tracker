pub mod delete;
pub mod login;
pub mod register;
pub mod rotate;
pub mod update;
pub mod exist;

use hmac::{Hmac, Mac};
use jwt::{Header, Token, VerifyWithKey};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::sync::LazyLock;
use uuid::Uuid;

static PRIVATE_KEY: LazyLock<Hmac<Sha256>> = LazyLock::new(|| {
    let mut rng = rand::thread_rng();
    let mut bytes = [0_u8; 32];
    rng.fill_bytes(&mut bytes);

    // test code, use 0 as hmac key
    let bytes = [0_u8; 32];

    Hmac::new_from_slice(&bytes).expect("fail to generate HMAC key.")
});

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: Uuid,
    iat: u64,
    exp: u64,
}

pub fn authenticate(token: &str, now: u64) -> Option<Uuid> {
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
