use super::schema;
use crate::hasura::creator::Creator;
use crate::imager::Image;

use chrono::{DateTime, Utc};
use cynic::{FragmentArguments, InputObject, QueryFragment};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(QueryFragment, Deserialize, Debug, Clone, PartialEq)]
#[cynic(schema_path = "schema.gql", query_module = "schema", graphql_type = "Quizzes")]
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

#[derive(QueryFragment, Validate, Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cynic(schema_path = "schema.gql", query_module = "schema", graphql_type = "quizzes")]
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
        Self {
            title: quiz.title,
            public: quiz.public,
            description: quiz.description,
            explanation: quiz.explanation,
            image: quiz.image.map(Image::from_url),
        }
    }
}
