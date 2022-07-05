use chrono::{DateTime, Utc};
use serde::Deserialize;
use validator::Validate;

use crate::graphql::creator::Creator;
use crate::Image;

impl hasura::Encode for Image {
    fn encode(&self) -> String {
        format!("\\\"{}\\\"", self.url().unwrap())
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq, hasura::Object, hasura::Pk, hasura::Encode)]
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
}

#[derive(Validate, Default, Debug, Clone, PartialEq, Deserialize, hasura::Encode)]
pub struct DraftQuiz {
    #[validate(length(min = 1, message = "Quiz must have a name."))]
    pub title: String,

    pub description: String,
    pub explanation: String,
    pub image: Image,
    pub public: bool,

    pub creator_id: String,
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
        }
    }
}
