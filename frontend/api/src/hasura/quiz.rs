use chrono::{DateTime, Utc};
use serde::Deserialize;
use validator::Validate;

use crate::hasura::creator::Creator;
use crate::imager::Image;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Quiz {
    pub id: u64,
    pub public: bool,

    pub title: String,
    pub description: String,
    pub explanation: String,
    pub image: Option<String>,

    pub created_at: DateTime<Utc>,
    pub creator: Creator,
}

#[derive(Validate, Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DraftQuiz {
    pub public: bool,

    #[validate(length(min = 1, message = "Quiz must have a name."))]
    pub title: String,
    pub description: String,
    pub explanation: String,
    pub image: Option<Image>,
}

impl From<Quiz> for DraftQuiz {
    fn from(quiz: Quiz) -> Self {
        let name = format!("{}.jpg", quiz.title.to_lowercase());
        Self {
            title: quiz.title,
            public: quiz.public,
            description: quiz.description,
            explanation: quiz.explanation,
            image: quiz.image.map(|url| Image::from_url(url, name)),
        }
    }
}
