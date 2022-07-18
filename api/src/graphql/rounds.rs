use derive_more::Display;
use hasura::{Object, Pk};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use strum::EnumIter;
use validator::{Validate, ValidationError};

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
}

impl Default for PointChoices {
    fn default() -> Self {
        Self::One
    }
}

#[derive(Serialize_repr, Deserialize_repr, Display, EnumIter, Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GuessChoices {
    #[display(fmt = "1")]
    One = 1,
    #[display(fmt = "2")]
    Two = 2,
    #[display(fmt = "3")]
    Three = 3,
    #[display(fmt = "infinite")]
    Infinite = 0,
}

impl Default for GuessChoices {
    fn default() -> Self {
        Self::Infinite
    }
}

fn validate_image(image: &Image) -> Result<(), ValidationError> {
    if image.is_none() {
        Err(ValidationError::new("Image must not be empty."))
    } else {
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Object, Pk)]
#[object(name = "rounds", pk = "quiz_id", pk = "index")]
pub struct Round {
    pub quiz_id: u32,
    pub index: u32,

    pub answer: String,
    pub points: PointChoices,
    pub guesses: GuessChoices,
    pub speed: Option<f64>,
    pub image: Image,
}

impl Round {
    pub fn from_draft(draft: DraftRound, quiz_id: u32, index: u32) -> Self {
        Self {
            quiz_id,
            index,
            answer: draft.answer,
            points: draft.points,
            guesses: draft.guesses,
            speed: draft.speed,
            image: draft.image,
        }
    }
}

#[derive(Validate, Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct DraftRound {
    #[validate(length(min = 1, message = "Round must have an answer."))]
    #[validate(length(max = 32, message = "Answer cannot exceed 32 characters."))]
    pub answer: String,

    pub points: PointChoices,
    pub guesses: GuessChoices,
    pub speed: Option<f64>,

    #[validate(custom = "validate_image")]
    pub image: Image,
}

impl From<Round> for DraftRound {
    fn from(round: Round) -> Self {
        Self {
            answer: round.answer,
            points: round.points,
            guesses: round.guesses,
            speed: round.speed,
            image: round.image,
        }
    }
}
