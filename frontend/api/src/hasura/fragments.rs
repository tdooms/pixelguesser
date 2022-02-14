use super::schema;
use crate::Image;
use chrono::{DateTime, Utc};
use cynic::{impl_scalar, serde_json, DecodeError, FragmentArguments, MutationBuilder, Scalar};

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
struct QuizzesPkColumnsInput {
    id: u64,
}

#[derive(cynic::FragmentArguments, Debug)]
struct UpdateQuizArgs {
    quiz_id: QuizzesPkColumnsInput,
    draft: Option<DraftQuiz>,
}

#[derive(cynic::FragmentArguments, Debug)]
struct DeleteQuizArgs {
    quiz_id: u64,
}

#[derive(cynic::QueryFragment, serde::Deserialize, Debug, Clone, PartialEq)]
#[cynic(schema_path = "schema.gql", query_module = "schema", graphql_type = "Quizzes")]
pub struct QuizId {
    pub id: u64,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema_path = "schema.gql", schema_module = "schema", graphql_type = "query_root")]
struct Quizzes {
    quizzes: Vec<Quiz>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema_path = "schema.gql",
    schema_module = "schema",
    graphql_type = "mutation_root",
    argument_struct = "DeleteQuizArgs"
)]
struct DeleteQuizzesByPk {
    #[arguments(id = & args.quiz_id)]
    delete_quizzes_by_pk: Option<QuizId>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema_path = "schema.gql",
    schema_module = "schema",
    graphql_type = "mutation_root",
    argument_struct = "UpdateQuizArgs"
)]
struct UpdateQuizzesByPk {
    #[arguments(pk_columns = &args.quiz_id, _set = &args.draft)]
    update_quizzes_by_pk: Option<QuizId>,
}
