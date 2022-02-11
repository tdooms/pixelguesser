use super::schema;
use crate::hasura::creator::Creator;
use crate::imager::Image;
use crate::{DraftRound, Round};
use chrono::{DateTime, Utc};
use cynic::{FragmentArguments, QueryFragment};
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

#[derive(Debug, Clone, PartialEq)]
pub struct FullQuiz {
    pub id: u64,
    pub public: bool,

    pub title: String,
    pub description: String,
    pub explanation: String,
    pub image: Option<String>,

    pub created_at: DateTime<Utc>,
    pub creator: Creator,
    pub rounds: Vec<Round>,
}

#[derive(Validate, Default, Debug, Clone, PartialEq)]
pub struct DraftQuiz {
    pub public: bool,

    #[validate(length(min = 1, message = "Quiz must have a name."))]
    pub title: String,
    pub description: String,
    pub explanation: String,
    pub image: Option<Image>,
}

#[derive(FragmentArguments, Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct GqlQuiz {
    pub public: bool,
    pub title: String,
    pub description: String,
    pub explanation: String,
    pub image: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct FullDraftQuiz {
    pub rounds: Vec<DraftRound>,
    pub quiz: DraftQuiz,
}

impl From<FullQuiz> for FullDraftQuiz {
    fn from(full: FullQuiz) -> Self {
        let rounds = full.rounds.iter().cloned().map(Into::into).collect();
        let quiz = full.into();
        FullDraftQuiz { rounds, quiz }
    }
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

impl From<DraftQuiz> for GqlQuiz {
    fn from(quiz: DraftQuiz) -> Self {
        Self {
            title: quiz.title,
            public: quiz.public,
            description: quiz.description,
            explanation: quiz.explanation,
            image: quiz.image.as_ref().map(|x| x.url()).flatten(),
        }
    }
}
