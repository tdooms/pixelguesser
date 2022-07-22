use crate::{Image, User};

use chrono::{DateTime, Utc};
use derive_more::Display;
use hasura::{Data, Object, Pk};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr as Derepr, Serialize_repr as Serepr};
use strum::EnumIter;
use validator::{Validate, ValidationError};

fn validate_image(image: &Image) -> Result<(), ValidationError> {
    if image.is_none() {
        Err(ValidationError::new("Image must not be empty."))
    } else {
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Object, Pk)]
#[object(name = "users", pk = "id")]
pub struct Creator {
    pub id: String,
    pub name: String,
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
    pub image: Image,

    pub created_at: DateTime<Utc>,

    #[object(expand)]
    pub creator: Creator,

    #[object(expand)]
    pub tags: Vec<Tag>,
}

#[derive(Validate, Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DraftQuiz {
    #[validate(length(min = 1, message = "Quiz must have a name."))]
    #[validate(length(max = 32, message = "Name cannot exceed 32 characters."))]
    pub title: String,

    pub description: String,
    pub explanation: String,
    pub image: Image,
    pub public: bool,

    #[serde(default)]
    pub creator_id: String,

    #[serde(skip_serializing)]
    pub tags: Data<Vec<DraftTag>>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FullQuiz {
    pub quiz: Quiz,
    pub rounds: Vec<Round>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub struct FullDraftQuiz {
    #[serde(flatten)]
    pub quiz: DraftQuiz,
    pub rounds: Vec<DraftRound>,
}

#[derive(Serepr, Derepr, Display, EnumIter, Clone, Copy, Debug, PartialEq, Default)]
#[repr(u8)]
pub enum PointChoices {
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
pub enum GuessChoices {
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

impl From<Tag> for DraftTag {
    fn from(tag: Tag) -> Self {
        Self { value: tag.value }
    }
}

impl From<FullQuiz> for FullDraftQuiz {
    fn from(full: FullQuiz) -> Self {
        let rounds = full.rounds.iter().cloned().map(Into::into).collect();
        FullDraftQuiz { rounds, quiz: full.quiz.into() }
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
            creator_id: quiz.creator.id,
            tags: Data { data: quiz.tags.into_iter().map(DraftTag::from).collect() },
        }
    }
}

impl From<User> for Creator {
    fn from(user: User) -> Self {
        Self { id: user.sub, name: user.nickname }
    }
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
