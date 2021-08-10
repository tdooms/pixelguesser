use crate::entities::{NewQuiz, RoundDiff, ScoreDiff, Stage};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Request {
    HostSession { quiz_id: u64 },
    ManageSession { session_id: u64 },
    LeaveSession { session_id: u64 },
    StopSession { session_id: u64 },

    ReadPlayers { session_id: u64 },
    CreatePlayer { session_id: u64, name: String },
    DeletePlayer { session_id: u64, player_id: u64 },

    UpdateScores { session_id: u64, diff: ScoreDiff },
    UpdateStage { session_id: u64, stage: Stage },
}
