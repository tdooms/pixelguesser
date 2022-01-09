use super::{exec, AffectedRows, Kind};
use crate::error::Error;
use crate::structs::ImageData;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use strum::EnumIter;
use validator::Validate;

pub const ROUND_FIELDS: &str = "round_id quiz_id index answer points guesses speed image_url";

#[derive(serde::Deserialize, Debug)]
pub struct SaveRoundsData {
    pub delete_rounds: AffectedRows,
    pub insert_rounds: AffectedRows,
}

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

#[derive(Validate, Serialize, Debug, Clone, Default, PartialEq)]
pub struct DraftRound {
    #[validate(length(min = 1))]
    pub answer: String,
    pub points: PointChoices,
    pub guesses: GuessChoices,
    pub speed: Option<f64>,
    pub image: Option<ImageData>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Round {
    pub round_id: u64,
    pub quiz_id: u64,
    pub index: u64,

    // copied from draft
    pub answer: String,
    pub points: PointChoices,
    pub guesses: GuessChoices,
    pub speed: Option<f64>,
    pub image: String,
}

pub async fn save_rounds(quiz_id: u64, rounds: &[DraftRound]) -> Result<(u64, u64), Error> {
    // TODO: parallelize
    for round in rounds {
        if let Some(image) = &round.image {
            image.upload().await?
        }
    }

    let objects = serde_json::to_string(&rounds).unwrap();
    let str = format!("delete_rounds(where: {{ quiz_id: (_eq: {}) }} ) insert_rounds(objects: {}) {{ affected_rows }}", quiz_id, objects);

    let data: SaveRoundsData = exec(Kind::Mutation(&str)).await?;
    Ok((data.delete_rounds.affected_rows, data.insert_rounds.affected_rows))
}
