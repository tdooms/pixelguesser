use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, EnumIter)]
pub enum Action {
    /// Adds a new player with given name
    AddPlayer(String),
    /// Removes a player with given name
    RemovePlayer(String),
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
    /// Give a rating to the quiz
    GiveRating(u64),
    /// Leave the session
    Leave,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Player {
    pub name: String,
    pub score: u64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Stage {
    Lobby,
    Playing { round: usize, paused: bool },
    Revealed { round: usize },
    Ranking { round: usize },
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
    pub fn update(&self, action: Action, rounds: usize) -> Option<Self> {
        let mut copy = (*self).clone();
        match (self.stage, action) {
            (Stage::Lobby, Action::AddPlayer(name)) => copy.players.push(Player { name, score: 0 }),
            (Stage::Lobby, Action::Start) => {
                // TODO: check if quiz is empty
                copy.stage = Stage::Playing { round: 0, paused: false }
            }
            (Stage::Playing { round, paused: true }, Action::Resume) => {
                copy.stage = Stage::Playing { round, paused: false }
            }
            (Stage::Playing { round, paused: false }, Action::Pause) => {
                copy.stage = Stage::Playing { round, paused: true }
            }
            (Stage::Playing { round, .. }, Action::Guessed(name, points)) => {
                for player in &mut copy.players {
                    if player.name == name {
                        player.score += points
                    }
                }
                copy.stage = Stage::Revealed { round }
            }
            (Stage::Playing { round, .. }, Action::Reveal) => {
                copy.stage = Stage::Revealed { round }
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

    pub fn actions(&self, rounds: usize) -> Vec<Action> {
        // This is far from the smartest ways to do this, but it's the least error prone
        Action::iter()
            .filter_map(|action| self.update(action.clone(), rounds).map(|_| action))
            .collect()
    }
}
