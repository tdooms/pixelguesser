use serde::{Deserialize, Serialize};

use crate::Stage;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Player {
    pub name: String,
    pub score: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Session {
    pub quiz_id: u64,

    pub stage: Stage,
    pub players: Vec<Player>,

    pub has_manager: bool,
    pub has_host: bool,
}
