use super::{exec, AffectedRows, Kind};
use crate::error::Error;
use crate::graphql::{quiz, ImageData};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use strum::EnumIter;
use validator::Validate;

pub const ROUND_FIELDS: &str = "round_id quiz_id index answer points guesses speed image";

#[derive(serde::Deserialize, Debug)]
pub struct SaveRoundsData {
    pub delete_rounds: AffectedRows,
    pub insert_rounds: AffectedRows,
}

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

impl From<Round> for DraftRound {
    fn from(round: Round) -> Self {
        Self {
            answer: round.answer,
            points: round.points,
            guesses: round.guesses,
            speed: round.speed,
            image: Some(ImageData::from_url(round.image)),
        }
    }
}

fn serialize(quiz_id: u64, index: usize, draft: &DraftRound) -> String {
    let image = draft.image.as_ref().map(|x| format!(", image: \\\"{}\\\"", x)).unwrap_or_default();
    let speed = draft.speed.as_ref().map(|x| format!(", speed: \\\"{}\\\"", x)).unwrap_or_default();
    format!(
        "quiz_id:\\\"{}\\\",index:\\\"{}\\\",answer:\\\"{}\\\",guesses:\\\"{}\\\",points:\\\"{}\\\"{}{}",
        quiz_id, index, draft.answer, draft.guesses as u8, draft.points as u8, speed, image
    )
}

pub async fn save_rounds(
    quiz_id: u64,
    mut rounds: Vec<DraftRound>,
) -> Result<Vec<DraftRound>, Error> {
    // TODO: parallelize
    for round in &mut rounds {
        if let Some(image) = &mut round.image {
            image.upload().await?
        }
    }
    let serialized: Vec<_> =
        rounds.iter().enumerate().map(|(idx, round)| serialize(quiz_id, idx, round)).collect();

    let objects = format!("[{{ {} }}]", serialized.join("},{"));
    let str = format!(
        "\
    delete_rounds(where: {{ quiz_id: {{ _eq: \\\"{}\\\" }} }} ) {{affected_rows}}\
    insert_rounds(objects: {}) {{ affected_rows }}",
        quiz_id, objects
    );

    let _: SaveRoundsData = exec(Kind::Mutation(&str)).await?;
    Ok(rounds)
}
