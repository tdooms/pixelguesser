use derive_more::Display;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use strum::EnumIter;
use validator::Validate;

use crate::Image;

#[derive(Serialize_repr, Deserialize_repr, Display, EnumIter, Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PointChoices {
    #[display(fmt = "0")]
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

#[derive(Validate, Debug, Clone, Default, PartialEq)]
pub struct DraftRound {
    #[validate(length(min = 1))]
    pub answer: String,
    pub points: PointChoices,
    pub guesses: GuessChoices,
    pub speed: Option<f64>,
    pub image: Option<Image>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Round {
    pub quiz_id: u64,
    pub index: u64,

    // copied from draft
    pub answer: String,
    pub points: PointChoices,
    pub guesses: GuessChoices,
    pub speed: Option<f64>,
    pub image: String,
}

impl From<Round> for DraftRound {
    fn from(round: Round) -> Self {
        Self {
            answer: round.answer,
            points: round.points,
            guesses: round.guesses,
            speed: round.speed,
            image: Some(Image::from_url(round.image, String::default())),
        }
    }
}
