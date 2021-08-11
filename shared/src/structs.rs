use serde::{Deserialize, Serialize};

use crate::Stage;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Player {
    pub name: String,
    pub score: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Session {
    pub stage: Stage,
    pub players: Vec<Player>,
}