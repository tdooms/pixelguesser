extern crate core;

use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use strum::EnumIter;

use serde::{Deserialize, Serialize};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("could not parse request {0}")]
    CouldNotParse(#[from] serde_json::Error),

    #[error("the request is not a text message")]
    NonText,

    #[error("already a manager present")]
    DuplicateManager,

    #[error("already a host present")]
    DuplicateHost,

    #[error("session not found")]
    SessionNotFound,
}

#[derive(EnumIter, Copy, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Mode {
    Couch,
    Online,
    Solo,
}

impl Display for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Couch => write!(f, "couch"),
            Mode::Online => write!(f, "online"),
            Mode::Solo => write!(f, "solo"),
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Player {
    pub score: i64,
    pub streak: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Participant {
    Manager,
    Host,
    Player(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Action {
    Join(Participant),

    Add(String),
    Remove(String),

    Start,
    Next,
    Finish,

    Stage(Stage),
    Score(String, i64),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Stage {
    Info,
    Running,
    Paused,
    Revealing,
    Revealed,
    Editing,
    Scores,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Default)]
pub enum Phase {
    #[default]
    Lobby,
    Playing {
        round: usize,
        stage: Stage,
    },
    Finished,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Session {
    pub mode: Mode,
    pub quiz: u32,

    pub players: HashMap<String, Player>,
    pub phase: Phase,
    pub round: Option<usize>,
    pub participants: HashMap<Participant, u32>,
}

impl Session {
    pub fn new(quiz: u32, mode: Mode) -> Self {
        let players = HashMap::new();
        let phase = Phase::default();
        let participants = HashMap::new();

        Self { mode, quiz, players, phase, round: None, participants }
    }

    pub fn update(&mut self, action: Action, id: u32) -> Result<(), Error> {
        match (action, self.phase) {
            (Action::Join(participant), _) => {
                match (self.participants.contains_key(&participant), participant) {
                    (true, Participant::Manager) => return Err(Error::DuplicateManager),
                    (true, Participant::Host) => return Err(Error::DuplicateHost),
                    (_, participant) => self.participants.insert(participant, id),
                };
            }
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
            (Action::Score(name, score), Phase::Playing { round, .. }) => {
                match self.players.get_mut(&name) {
                    Some(player) => player.score += score,
                    None => (),
                }
                self.phase = Phase::Playing { round, stage: Stage::Revealing };
            }
            _ => (),
        }
        Ok(())
    }
}
