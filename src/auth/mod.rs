mod database;
pub mod insert;
pub mod login;
pub mod check;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum UserGroup {
    Viewer,
    Editor,
}

pub use login::verify;
