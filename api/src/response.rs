use serde::{Deserialize, Serialize};

use crate::entities::{Player, Quiz, Round, ScoreDiff, Session, Stage};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Error {
    SessionDoesNotExist(u64),
    PlayerDoesNotExist(u64),
    SessionCreationFailed,
    FaultyRequest,
    InternalServerError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Response {
    HostedSession(u64, Session),
    ManagedSession(u64, Session),
    LeftSession(u64),
    StoppedSession(u64),

    ReadPlayers(HashMap<u64, Player>),
    CreatedPlayer(u64, String),
    DeletedPlayer(u64, String),

    UpdatedManager(u64, bool),

    UpdatedScores(u64, ScoreDiff),
    UpdatedStage(u64, Stage),

    Error(Error),
}
