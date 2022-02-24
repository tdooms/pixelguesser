use super::schema;

use chrono::{DateTime, Utc};
use cynic::impl_scalar;

use crate::hasura::quiz::*;

impl_scalar!(u64, schema::Bigint);
impl_scalar!(DateTime<Utc>, schema::Timestamptz);
impl_scalar!(DraftQuiz, schema::QuizzesSetInput);

#[derive(cynic::InputObject, Debug)]
#[cynic(
    schema_path = "schema.gql",
    schema_module = "schema",
    graphql_type = "quizzes_pk_columns_input",
    rename_all = "snake_case"
)]
pub struct QuizzesPkColumnsInput {
    pub id: u64,
}

#[derive(cynic::FragmentArguments, Debug)]
pub struct UpdateQuizArgs {
    pub quiz_id: QuizzesPkColumnsInput,
    pub draft: Option<DraftQuiz>, // TODO: remove option
}

#[derive(cynic::FragmentArguments, Debug)]
pub struct DeleteQuizArgs {
    pub quiz_id: u64,
}

#[derive(cynic::FragmentArguments, Debug)]
pub struct InsertQuizzesOneArgs {
    pub draft: DraftQuiz,
}

#[derive(cynic::QueryFragment, serde::Deserialize, Debug, Clone, PartialEq)]
#[cynic(schema_path = "schema.gql", query_module = "schema", graphql_type = "Quizzes")]
pub struct QuizId {
    pub id: u64,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema_path = "schema.gql", schema_module = "schema", graphql_type = "query_root")]
pub struct Quizzes {
    pub quizzes: Vec<Quiz>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema_path = "schema.gql",
    schema_module = "schema",
    graphql_type = "mutation_root",
    argument_struct = "DeleteQuizArgs"
)]
pub struct DeleteQuizzesByPk {
    #[arguments(id = & args.quiz_id)]
    pub delete_quizzes_by_pk: Option<QuizId>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema_path = "schema.gql",
    schema_module = "schema",
    graphql_type = "mutation_root",
    argument_struct = "UpdateQuizArgs"
)]
pub struct UpdateQuizzesByPk {
    #[arguments(pk_columns = &args.quiz_id, _set = &args.draft)]
    pub update_quizzes_by_pk: Option<Quiz>,
}

// #[derive(cynic::QueryFragment, Debug)]
// #[cynic(
//     schema_path = "schema.gql",
//     schema_module = "schema",
//     graphql_type = "mutation_root",
//     argument_struct = "InsertQuizzesOneArgs"
// )]
// pub struct InsertQuizzesOne {
//     #[arguments(object = &args.draft)]
//     pub insert_quizzes_one: Option<QuizId>,
// }
