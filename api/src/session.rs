use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::entities::{Player, Quiz, Round};

#[derive(Debug, Clone, Serialize, Deserialize, Copy, Eq, PartialEq)]
pub enum Status {
    Playing { paused: bool },
    Revealing,
    Revealed,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Stage {
    Initial,
    Round { round: usize, status: Status },
    Scores { round: usize },
    Finish,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Session {
    pub stage: Stage,
    pub quiz: Quiz,
    pub rounds: Vec<Round>,
    pub players: HashMap<u64, Player>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Action {
    Start,
    Pause,
    Resume,
    Reveal,
    Scores,
    Next,
    Finish,
    Leave,
}

impl Stage {
    pub fn perform(&self, action: Action, rounds: usize) -> Option<Stage> {
        match (*self, action) {
            (Stage::Initial, Action::Start) if rounds == 0 => Some(Stage::Finish),
            (Stage::Initial, Action::Start) => {
                Some(Stage::Round { round: 0, status: Status::Playing { paused: false } })
            }
            (Stage::Round { round, status: Status::Playing { paused: false } }, Action::Pause) => {
                Some(Stage::Round { round, status: Status::Playing { paused: true } })
            }
            (Stage::Round { round, status: Status::Playing { paused: true } }, Action::Resume) => {
                Some(Stage::Round { round, status: Status::Playing { paused: false } })
            }
            (Stage::Round { round, status: Status::Playing { .. } }, Action::Reveal) => {
                Some(Stage::Round { round, status: Status::Revealing })
            }
            (Stage::Round { round, status: Status::Revealed }, Action::Finish)
                if round >= rounds - 1 =>
            {
                Some(Stage::Finish)
            }
            (
                Stage::Round { round, status: Status::Revealed } | Stage::Scores { round },
                Action::Next,
            ) if round < rounds - 1 => {
                Some(Stage::Round { round: round + 1, status: Status::Playing { paused: false } })
            }
            (Stage::Round { round, status: Status::Revealed }, Action::Scores)
                if round < rounds - 1 =>
            {
                Some(Stage::Scores { round })
            }
            _ => None,
        }
    }

    pub fn actions(&self, rounds: usize) -> Vec<Action> {
        match *self {
            Stage::Initial => vec![Action::Start],
            Stage::Round { round: _, status: Status::Playing { paused } } => match paused {
                true => vec![Action::Reveal, Action::Resume],
                false => vec![Action::Reveal, Action::Pause],
            },
            Stage::Round { round, status: Status::Revealed } => match round >= rounds - 1 {
                true => vec![Action::Finish],
                false => vec![Action::Next, Action::Scores],
            },
            Stage::Round { round: _, status: Status::Revealing } => vec![],
            Stage::Scores { round } if round < rounds - 1 => vec![Action::Next],
            _ => vec![Action::Leave],
        }
    }
}
