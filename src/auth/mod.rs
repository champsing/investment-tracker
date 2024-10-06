mod database;
mod services;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum UserGroup {
    Viewer,
    Editor,
}

pub use services::*;
