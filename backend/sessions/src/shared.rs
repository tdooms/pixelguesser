use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Player {
    score: i64,
    streak: i64,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct State {
    pub players: HashMap<String, Player>,
    pub status: Status,
    pub round: Option<usize>,
    pub master: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Action {
    Add(String),
    Remove(String),

    Start,
    Next,

    Status(Status),
    Score(String, i64),
    Master(u64),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Status {
    Playing,
    Paused,
    Revealing,
    Revealed,
}

impl Default for Status {
    fn default() -> Self {
        Status::Playing
    }
}

impl State {
    pub fn update(&mut self, action: Action) {
        match action {
            Action::Add(name) => {
                self.players.insert(name, Player::default());
            }
            Action::Remove(name) => {
                self.players.remove(&name);
            }
            Action::Start => {
                self.round = Some(0);
                self.status = Status::Playing;
            }
            Action::Status(status) => {
                self.status = status;
            }
            Action::Next => {
                self.round = Some(self.round.unwrap() + 1);
                self.status = Status::Playing;
            }
            Action::Master(id) => {
                self.master = Some(id);
            }
            Action::Score(name, score) => {
                let player = self.players.get_mut(&name).unwrap();
                player.score += score;
            }
        }
    }
}
