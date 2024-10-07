mod database;
mod services;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum UserGroup {
    Viewer,
    Editor,
}

pub use database::init;
pub use services::insert::handler as insert;
pub use services::login::handler as login;
pub use services::refresh::handler as refresh;
