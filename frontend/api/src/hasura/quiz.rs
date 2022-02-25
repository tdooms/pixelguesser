use chrono::{DateTime, Utc};
use cynic::{InputObject, QueryFragment};
use serde::Deserialize;
use validator::Validate;

use crate::hasura::creator::Creator;
use crate::imager::Image;
use crate::Resolution;

use super::schema;

#[derive(QueryFragment, Deserialize, Debug, Clone, PartialEq)]
#[cynic(schema_path = "schema.gql", query_module = "schema", graphql_type = "quizzes")]
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

#[derive(InputObject, Default, Debug, Clone, PartialEq, Deserialize)]
#[cynic(schema_path = "schema.gql", query_module = "schema", graphql_type = "quizzes_insert_input")]
pub struct GqlDraftQuiz {
    pub public: bool,

    pub title: String,
    pub description: String,
    pub explanation: String,
    pub image: Option<String>,
}

impl From<DraftQuiz> for GqlDraftQuiz {
    fn from(quiz: DraftQuiz) -> Self {
        Self {
            title: quiz.title,
            public: quiz.public,
            description: quiz.description,
            explanation: quiz.explanation,
            image: quiz.image.map(|img| img.src(Resolution::Max)),
        }
    }
}

// TODO post error on github
// required because of the requirements on the impl of `InputType<std::string::String, Nullable<NamedType>>` for `&std::option::Option<Image>`
// #[derive(InputObject, Validate, Default, Debug, Clone, PartialEq, Deserialize)]
//          ^^^^^^^^^^^ required by this bound in `image`
//pub image: Option<Image>,
//    ----- required by a bound in this

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
