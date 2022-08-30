use crate::Image;

use chrono::{DateTime, Utc};
use derive_more::Display;
use hasura::{Data, Object, Pk};
use serde::{Deserialize, Deserializer, Serialize};
use serde_repr::{Deserialize_repr as Derepr, Serialize_repr as Serepr};
use strum::EnumIter;
use validator::{Validate, ValidationError};

fn validate_image(image: &Image) -> Result<(), ValidationError> {
    if image.is_empty() {
        Err(ValidationError::new("Image must not be empty."))
    } else {
        Ok(())
    }
}

fn default_speed() -> u64 {
    100
}

fn skip_empty<T: Serialize>(x: &Data<Vec<T>>) -> bool {
    x.data.is_empty()
}

fn strip_data<'de, T: Deserialize<'de> + Serialize, D: Deserializer<'de>>(
    deser: D,
) -> Result<Data<Vec<T>>, D::Error> {
    Ok(Data { data: Vec::<T>::deserialize(deser)? })
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

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct DraftTag {
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Object, Pk)]
#[object(name = "tags", pk = "quiz_id", pk = "value")]
pub struct Tag {
    pub quiz_id: u32,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Object, Pk)]
#[object(name = "quizzes", pk = "id", draft = "DraftQuiz")]
pub struct Quiz {
    pub id: u32,
    pub public: bool,
    pub complete: bool,
    pub title: String,
    pub description: String,
    pub explanation: String,
    pub created_at: DateTime<Utc>,

    #[serde(default)]
    pub image: Image,

    #[object(expand)]
    pub creator: User,

    #[object(expand)]
    #[serde(default)]
    pub tags: Vec<Tag>,

    #[object(expand)]
    #[serde(default)]
    pub rounds: Vec<Round>,
}

#[derive(Validate, Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DraftQuiz {
    #[validate(length(min = 1, message = "Quiz must have a title."))]
    #[validate(length(max = 32, message = "Title cannot exceed 32 characters."))]
    pub title: String,

    #[serde(default)]
    pub creator_id: Option<String>,

    pub description: String,
    pub explanation: String,
    pub public: bool,

    #[serde(default)]
    pub image: Image,

    #[serde(skip_serializing_if = "skip_empty")]
    #[serde(deserialize_with = "strip_data")]
    #[serde(default)]
    pub tags: Data<Vec<DraftTag>>,

    #[serde(skip_serializing_if = "skip_empty")]
    #[serde(deserialize_with = "strip_data")]
    #[serde(default)]
    pub rounds: Data<Vec<DraftRound>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Object, Pk)]
#[object(name = "rounds", pk = "quiz_id", pk = "index")]
pub struct Round {
    pub quiz_id: u32,
    pub index: u32,
    pub points: Points,
    pub guesses: Guesses,
    pub answer: String,
    pub speed: u64,
    pub algorithm: Algorithm,
    pub image: Image,
}

#[derive(Validate, Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct DraftRound {
    #[serde(default)]
    pub index: u32,

    #[validate(length(min = 1, message = "Round must have an answer."))]
    #[validate(length(max = 32, message = "Answer cannot exceed 32 characters."))]
    pub answer: String,

    pub points: Points,
    pub guesses: Guesses,

    #[serde(default = "default_speed")]
    pub speed: u64,

    #[serde(default)]
    pub algorithm: Algorithm,

    #[validate(custom = "validate_image")]
    pub image: Image,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Default, Object, Pk)]
#[object(name = "users", pk = "id")]
pub struct User {
    pub id: String,
    pub nickname: String,
    pub picture: String,
    pub email: String,
    pub email_verified: bool,
}

impl From<Tag> for DraftTag {
    fn from(tag: Tag) -> Self {
        Self { value: tag.value }
    }
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
            algorithm: draft.algorithm,
        }
    }
}

impl From<Quiz> for DraftQuiz {
    fn from(quiz: Quiz) -> Self {
        Self {
            title: quiz.title,
            public: quiz.public,
            description: quiz.description,
            explanation: quiz.explanation,
            image: quiz.image,
            creator_id: Some(quiz.creator.id),
            tags: Data { data: quiz.tags.into_iter().map(DraftTag::from).collect() },
            rounds: Data { data: quiz.rounds.into_iter().map(DraftRound::from).collect() },
        }
    }
}

impl From<Round> for DraftRound {
    fn from(round: Round) -> Self {
        Self {
            index: round.index,
            answer: round.answer,
            points: round.points,
            guesses: round.guesses,
            speed: round.speed,
            image: round.image,
            algorithm: round.algorithm,
        }
    }
}
