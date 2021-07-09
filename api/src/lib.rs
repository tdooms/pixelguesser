use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quiz {
    pub quiz_id: i64,
    pub name: String,
    pub creator: String,
    pub description: String,
    pub image_url: String,
    pub time_created: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Round {
    pub round_id: i64,
    pub quiz_id: i64,
    pub image_url: String,
    pub points: i64,
    pub guesses: i64,
    pub speed: Option<f64>,
    pub answer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub score: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreChange {
    pub player_id: u64,
    pub change: i64,
    pub reason: String,
}

pub type ScoreDiff = Vec<ScoreChange>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionData {
    pub stage: Stage,
    pub quiz: Quiz,
    pub rounds: Vec<Round>,
    pub players: HashMap<u64, Player>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy, Eq, PartialEq)]
pub enum Status {
    Paused,
    Playing,
    Revealing,
    Revealed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Stage {
    Initial,
    Round { round: usize, status: Status },
    Scores { round: usize },
    Finish,
}

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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Reply {
    SessionCreated(Quiz, Vec<Round>),
    SessionJoined(SessionData),
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
