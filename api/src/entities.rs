use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Quiz {
    pub quiz_id: i64,
    pub name: String,
    pub creator: String,
    pub description: String,
    pub image_url: String,
    pub time_created: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Round {
    pub round_id: i64,
    pub quiz_id: i64,
    pub image_url: String,
    pub points: i64,
    pub guesses: i64,
    pub speed: Option<f64>,
    pub answer: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DraftRound {
    pub image_url: Option<String>,
    pub points: i64,
    pub guesses: i64,
    pub speed: Option<f64>,
    pub answer: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Player {
    pub name: String,
    pub score: i64,
}
