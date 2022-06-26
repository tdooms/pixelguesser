use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("could not parse request")]
    Parse(#[from] serde_json::Error),

    #[error("the request is not a text message")]
    NonText,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Player {
    pub score: i64,
    pub streak: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Action {
    Add(String),
    Remove(String),

    Start,
    Next,
    Finish,

    Stage(Stage),
    Score(String, i64),
    Master,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Stage {
    Info,
    Running,
    Paused,
    Revealing,
    Revealed,
    Scores,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Phase {
    Lobby,
    Playing { round: usize, stage: Stage },
    Finished,
}

impl Default for Phase {
    fn default() -> Self {
        Phase::Lobby
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Response {
    Update(Session),
    Error(String),
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Session {
    pub players: HashMap<String, Player>,
    pub phase: Phase,
    pub round: Option<usize>,
    pub master: Option<u32>,
}

impl Session {
    pub fn update(&mut self, action: Action, id: u32) {
        match (action, self.phase) {
            (Action::Add(name), Phase::Lobby) => {
                self.players.insert(name, Player::default());
            }
            (Action::Remove(name), Phase::Lobby) => {
                self.players.remove(&name);
            }
            (Action::Start, Phase::Lobby) => {
                self.phase = Phase::Playing { round: 0, stage: Stage::Info };
            }
            (Action::Stage(stage), Phase::Playing { round, .. }) => {
                self.phase = Phase::Playing { round, stage };
            }
            (Action::Next, Phase::Playing { round, .. }) => {
                self.phase = Phase::Playing { round: round + 1, stage: Stage::Info };
            }
            (Action::Finish, _) => {
                self.phase = Phase::Finished;
            }
            (Action::Master, _) => {
                // TODO: check is there is already a master
                self.master = Some(id);
            }
            (Action::Score(name, score), Phase::Playing { round, .. }) => {
                match self.players.get_mut(&name) {
                    Some(player) => player.score += score,
                    None => (),
                }

                self.phase = Phase::Playing { round, stage: Stage::Revealing };
            }
            _ => (),
        }
    }
}
