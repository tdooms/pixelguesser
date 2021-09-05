use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use self::Action::*;
use self::Stage::*;
use self::Status::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, EnumIter)]
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
    Ranking { round: usize },
    Finished,
}

impl Default for Stage {
    fn default() -> Self {
        Stage::Initial
    }
}

impl Stage {
    pub fn perform(&self, action: Action, rounds: usize) -> Option<Stage> {
        match (*self, action) {
            (Initial, Start) if rounds == 0 => Some(Finished),
            (Initial, Start) => Some(Round { round: 0, status: Playing { paused: false } }),
            (Round { round, status: Playing { paused: false } }, Pause) => {
                Some(Round { round, status: Playing { paused: true } })
            }
            (Round { round, status: Playing { paused: true } }, Resume) => {
                Some(Round { round, status: Playing { paused: false } })
            }
            (Round { round, status: Playing { .. } }, Reveal) => {
                Some(Round { round, status: Revealing })
            }
            (Round { round, status: Revealed }, Finish) if round >= rounds - 1 => Some(Finished),
            (Round { round, status: Revealed } | Ranking { round }, Next) if round < rounds - 1 => {
                Some(Round { round: round + 1, status: Playing { paused: false } })
            }
            (Round { round, status: Revealed }, Scores) if round < rounds - 1 => {
                Some(Ranking { round })
            }
            _ => None,
        }
    }
    pub fn actions(&self, rounds: usize) -> Vec<Action> {
        match *self {
            Initial => vec![Start],
            Round { round: _, status: Playing { paused: true } } => vec![Reveal, Resume],
            Round { round: _, status: Playing { paused: false } } => vec![Reveal, Pause],
            Round { round, status: Revealed } if round >= rounds - 1 => {
                vec![Finish]
            }
            Round { round: _, status: Revealed } => vec![Next, Scores],
            Round { round: _, status: Revealing } => vec![],
            Ranking { round } if round < rounds - 1 => vec![Next],
            _ => vec![Leave],
        }
    }
}
