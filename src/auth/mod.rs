mod database;
mod services;

use crate::constant::path;
use const_format::formatcp as const_format;
use serde::{Deserialize, Serialize};

const CREDENTIAL_DATABASE: &str = const_format!("{}/credential.db", path::DATA);

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum UserGroup {
    Viewer,
    Editor,
}

pub use database::init;
pub use services::all_users::handler as all_users;
pub use services::delete::handler as delete;
pub use services::login::handler as login;
pub use services::refresh::handler as refresh;
pub use services::upsert::handler as upsert;
pub use services::verify;
