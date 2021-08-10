use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScoreDiff {
    pub player_id: u64,
    pub change: i64,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Player {
    pub name: String,
    pub score: i64,
}
