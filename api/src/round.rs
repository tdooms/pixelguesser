use derive_more::Display;
use hasura::Hasura;
use serde::{Deserialize, Serialize};
use serde_repr::Deserialize_repr as Derepr;
use serde_repr::Serialize_repr as Serepr;
use strum::EnumIter;
use validator::{Validate, ValidationError};

use crate::Image;

fn validate(image: &Image) -> Result<(), ValidationError> {
    match image.is_empty() {
        true => Err(ValidationError::new("Image must not be empty.")),
        false => Ok(()),
    }
}

fn default_speed() -> u64 {
    100
}

#[derive(Serepr, Derepr, Display, EnumIter, Clone, Copy, Debug, PartialEq, Default)]
#[repr(u8)]
pub enum Points {
    #[display(fmt = "0")]
    None = 0,
    #[display(fmt = "1")]
    #[default]
    One = 1,
    #[display(fmt = "2")]
    Two = 2,
    #[display(fmt = "3")]
    Three = 3,
    #[display(fmt = "4")]
    Four = 4,
}

#[derive(Serepr, Derepr, Display, EnumIter, Clone, Copy, Debug, PartialEq, Default)]
#[repr(u8)]
pub enum Guesses {
    #[display(fmt = "1")]
    One = 1,
    #[display(fmt = "2")]
    Two = 2,
    #[display(fmt = "3")]
    Three = 3,
    #[display(fmt = "infinite")]
    #[default]
    Infinite = 0,
}

#[derive(Serepr, Derepr, Display, EnumIter, Clone, Copy, Debug, PartialEq, Default)]
#[repr(u8)]
pub enum Algorithm {
    #[default]
    #[display(fmt = "pixelate")]
    Pixelate = 0,
    #[display(fmt = "contrast")]
    Contrast = 1,
    #[display(fmt = "blur")]
    Blur = 2,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hasura, Validate, Default)]
#[hasura(table = "rounds")]
pub struct Round {
    #[hasura(pk = "u64")]
    pub quiz_id: Option<u64>,

    #[hasura(pk = "u64")]
    #[serde(default)]
    pub index: u64,

    #[validate(length(min = 1, message = "Round must have an answer."))]
    #[validate(length(max = 32, message = "Answer cannot exceed 32 characters."))]
    pub answer: String,

    pub points: Points,
    pub guesses: Guesses,

    #[serde(default = "default_speed")]
    pub speed: u64,

    #[serde(default)]
    pub algorithm: Algorithm,

    #[validate(custom = "validate")]
    pub image: Image,
}
