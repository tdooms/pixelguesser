use chrono::{DateTime, Utc};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use serde_repr::*;
use strum_macros::EnumIter;

#[derive(Serialize_repr, Deserialize_repr, Display, EnumIter, Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PointChoices {
    #[display(fmt = "none")]
    None = 0,
    #[display(fmt = "1")]
    One = 1,
    #[display(fmt = "2")]
    Two = 2,
    #[display(fmt = "3")]
    Three = 3,
    #[display(fmt = "4")]
    Four = 4,
    #[display(fmt = "5")]
    Five = 5,
}

impl Default for PointChoices {
    fn default() -> Self {
        Self::One
    }
}

#[derive(Serialize_repr, Deserialize_repr, Display, EnumIter, Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GuessChoices {
    #[display(fmt = "no limit")]
    Infinity = 0,
    #[display(fmt = "1")]
    One = 1,
    #[display(fmt = "2")]
    Two = 2,
    #[display(fmt = "3")]
    Three = 3,
}

impl Default for GuessChoices {
    fn default() -> Self {
        Self::Infinity
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Quiz {
    pub quiz_id: u64,
    pub name: String,
    pub description: String,
    pub creator: String,
    pub created_at: DateTime<Utc>,
    pub image_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DraftQuiz {
    pub name: String,
    pub description: String,
    pub creator: String,
    pub image_url: Option<String>,

    #[serde(skip)]
    pub image_local: Option<web_sys::File>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Round {
    pub answer: String,
    pub points: PointChoices,
    pub guesses: GuessChoices,
    pub image_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct DraftRound {
    pub points: PointChoices,
    pub guesses: GuessChoices,
    pub answer: String,
    pub image_url: Option<String>,

    #[serde(skip)]
    pub image_local: Option<web_sys::File>,
}
