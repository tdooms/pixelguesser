use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, EnumIter)]
pub enum Action {
    /// Adds a new player with given name
    Player(String),
    /// Starts the quiz after doing the setup
    Start,
    /// Pauses a round
    Pause,
    /// Resumes a round
    Resume,
    /// Reveals the image of the current round
    Reveal,
    /// Increments the point of a player   
    Guessed(String, u64),
    /// Show the current scores
    Scores,
    /// Proceed to next round
    Next,
    /// Leave the session
    Leave,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Player {
    name: String,
    score: u64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Stage {
    Lobby,
    Playing { round: u64, paused: bool },
    Revealed { round: u64 },
    Ranking { round: u64 },
    Finished,
    Left,
}

impl Default for Stage {
    fn default() -> Self {
        Stage::Lobby
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Session {
    pub stage: Stage,
    pub players: Vec<Player>,
    pub has_manager: bool,
}

impl Session {
    pub fn update(&self, action: Action, rounds: u64) -> Option<Self> {
        let mut copy = (*self).clone();
        match (self.stage, action) {
            (Stage::Lobby, Action::Player(name)) => copy.players.push(Player { name, score: 0 }),
            (Stage::Playing { round, paused: true }, Action::Resume) => {
                copy.stage = Stage::Playing { round, paused: false }
            }
            (Stage::Playing { round, paused: false }, Action::Pause) => {
                copy.stage = Stage::Playing { round, paused: true }
            }
            (Stage::Revealed { round }, Action::Next) if round >= rounds - 1 => {
                copy.stage = Stage::Finished
            }
            (Stage::Revealed { round }, Action::Scores) => copy.stage = Stage::Ranking { round },
            (Stage::Revealed { round }, Action::Next) => {
                copy.stage = Stage::Playing { round: round + 1, paused: false }
            }
            (Stage::Ranking { round }, Action::Next) => {
                copy.stage = Stage::Playing { round: round + 1, paused: false }
            }
            (Stage::Finished, Action::Leave) => copy.stage = Stage::Left,
            _ => return None,
        }
        Some(copy)
    }

    pub fn actions(&self, rounds: u64) -> Vec<Action> {
        // This is far from the smartest ways to do this, but it's the least error prone
        Action::iter()
            .filter_map(|action| self.update(action.clone(), rounds).map(|_| action))
            .collect()
    }
}
