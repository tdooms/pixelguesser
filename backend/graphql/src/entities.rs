use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Quiz {
    pub quiz_id: u64,
    pub name: String,
    pub description: String,
    pub author: String,
    pub creator: String,
    pub created: DateTime<Utc>,
    pub image_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NewQuiz {
    pub name: String,
    pub description: String,
    pub author: String,
    pub creator: String,
    pub image_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Round {
    pub answer: String,
    pub points: u64,
    pub guesses: u64,
    pub image_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DraftQuiz {
    pub name: Option<String>,
    pub description: Option<String>,
    pub author: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DraftRound {
    pub answer: Option<String>,
}
