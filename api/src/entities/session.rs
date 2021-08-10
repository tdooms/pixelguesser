use crate::entities::{Player, Stage};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Session {
    pub stage: Stage,
    pub players: HashMap<u64, Player>,
}
