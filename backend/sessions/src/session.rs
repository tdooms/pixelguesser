use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, EnumIter)]
pub enum Action {
    /// Adds a new player with given name
    AddPlayer(String),
    /// Removes a player with given name
    RemovePlayer(String),
    /// Starts the quiz after doing the setup
    StartQuiz,
    /// Pauses a round
    PauseRound,
    /// Resumes a round
    ResumeRound,
    /// Reveals the image of the current round
    RevealRound,
    /// Increments/decrements the point of a player
    UpdateScore(String, i64),
    /// Increments the point of a player and reveals
    CorrectGuess(String, i64),
    /// Proceed to next round
    NextRound,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Player {
    pub score: i64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Stage {
    Lobby,
    Playing { round: usize, paused: bool, revealing: bool },
    Finished,
}

impl Default for Stage {
    fn default() -> Self {
        Stage::Lobby
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Session {
    pub stage: Stage,
    pub players: HashMap<String, Player>,
}

impl Session {
    fn insert_player(&mut self, name: String) {
        // Try the name itself, if it already exists try increasingly high numbers
        if self.players.insert(name.clone(), Player::default()).is_some() {
            for i in 1..u64::MAX {
                let name = format!("{} #{}", name.clone(), i);
                if self.players.insert(name, Player::default()).is_none() {
                    break;
                }
            }
        }
    }

    pub fn update(&self, action: Action, rounds: usize) -> Option<Self> {
        let mut copy = (*self).clone();
        match (self.stage, action) {
            (Stage::Lobby, Action::AddPlayer(name)) => {
                // This always succeeds
                copy.insert_player(name)
            }
            (Stage::Lobby, Action::StartQuiz) if rounds == 0 => {
                // Cannot start quiz if no rounds
                copy.stage = Stage::Finished;
            }
            (Stage::Lobby, Action::StartQuiz) => {
                // Start at round 0 on start
                copy.stage = Stage::Playing { round: 0, paused: false, revealing: false }
            }
            (Stage::Playing { round, paused: true, revealing: false }, Action::ResumeRound) => {
                // Resume action if paused and not revealed
                copy.stage = Stage::Playing { round, paused: false, revealing: false }
            }
            (Stage::Playing { round, paused: false, revealing: false }, Action::PauseRound) => {
                // Pause action if paused and not revealed
                copy.stage = Stage::Playing { round, paused: true, revealing: false }
            }
            (Stage::Playing { round, .. }, Action::CorrectGuess(name, points)) => {
                // Grant score and start revealing on correct guess
                if let Some(player) = copy.players.get_mut(&name) {
                    player.score += points;
                }
                copy.stage = Stage::Playing { round, revealing: true, paused: false }
            }
            (Stage::Playing { round, revealing: false, .. }, Action::RevealRound) => {
                // Reveal without granting anyone points
                copy.stage = Stage::Playing { round, revealing: true, paused: false }
            }
            (Stage::Playing { round, revealing: true, .. }, Action::NextRound)
                if round >= rounds - 1 =>
            {
                // Finish action if quiz is done
                copy.stage = Stage::Finished
            }
            (Stage::Playing { round, revealing: true, .. }, Action::NextRound) => {
                // Next round action if quiz is not done
                copy.stage = Stage::Playing { round: round + 1, paused: false, revealing: false }
            }
            (Stage::Playing { .. }, Action::UpdateScore(name, points)) => {
                // Update score without changing anything else
                if let Some(player) = copy.players.get_mut(&name) {
                    player.score += points;
                }
            }
            (Stage::Lobby, Action::RemovePlayer(name)) => {
                // Remove player, no errors on non-existent
                copy.players.remove(&name);
            }
            _ => return None,
        }
        Some(copy)
    }

    pub fn actions(&self, rounds: usize) -> Vec<Action> {
        // Automatically check which actions are legal using the logic itself
        Action::iter()
            .filter_map(|action| self.update(action.clone(), rounds).map(|_| action))
            .collect()
    }
}
