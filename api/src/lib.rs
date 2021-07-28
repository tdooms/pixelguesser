use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub use crate::entities::*;
pub use crate::session::*;

mod entities;
mod session;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScoreChange {
    pub player_id: u64,
    pub change: i64,
    pub reason: String,
}

pub type ScoreDiff = Vec<ScoreChange>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Error {
    SessionDoesNotExist(u64),
    PlayerDoesNotExist(u64),
    SessionCreationFailed,
    FaultyRequest,
    InternalServerError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Get {
    Quizzes,
    Quiz { quiz_id: i64 },
    Stage { session_id: u64 },
    Players { session_id: u64 },
    CheckSession { session_id: u64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Fetch {
    Quizzes(Vec<Quiz>),
    Quiz(Quiz, Vec<Round>),
    Players(HashMap<u64, Player>),
    Stage(Stage),

    SessionAvailable(u64),
    SessionInvalid(u64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Post {
    StartSession { quiz_id: i64 },
    StopSession { session_id: u64 },

    JoinSession { session_id: u64 },
    LeaveSession { session_id: u64 },

    AddPlayer { session_id: u64, name: String },
    ChangeScores { session_id: u64, diff: ScoreDiff },
    ChangeStage { session_id: u64, stage: Stage },
    // UploadDraft { session_id: u64, quiz_id: i64, rounds: DraftRound },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Reply {
    SessionCreated(Quiz, Vec<Round>),
    SessionJoined(Session),
    SessionManaged,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Alert {
    ManagerJoined,
    ManagerLeft,
    SessionStopped,

    ScoreChanged(ScoreDiff),
    StageChanged(Stage),
    PlayerAdded(u64, String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Request {
    Get(Get),
    Post(Post),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Response {
    Fetch(Fetch),
    Reply(u64, Reply),
    Alert(u64, Alert),
    Error(Error),
}
