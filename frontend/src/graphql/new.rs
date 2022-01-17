// use super::schema;
// use crate::structs::ImageData;
// use chrono::{DateTime, Utc};
// use cynic::{impl_scalar, DecodeError, Scalar};
//
// impl_scalar!(u64, schema::Bigint);
// impl_scalar!(DateTime<Utc>, schema::Timestamptz);
//
// // #[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq)]
// // pub enum Bong {
// //     Whee,
// //     Whoo,
// // }
// //
// // impl Default for Bong {
// //     fn default() -> Self {
// //         Bong::Whoo
// //     }
// // }
// //
// // impl Scalar<String> for Bong {
// //     type Deserialize = String;
// //
// //     fn from_deserialize(x: Self::Deserialize) -> Result<Self, DecodeError> {
// //         Ok(Bong::Whoo)
// //     }
// // }
//
// #[derive(validator::Validate, Debug, Default, Clone, PartialEq)]
// pub struct DraftQuiz {
//     #[validate(length(min = 1))]
//     pub name: String,
//     pub description: String,
//     pub creator: String,
//     pub image: Option<ImageData>,
// }
//
// #[derive(cynic::QueryFragment, validator::Validate, Debug, Default, Clone, PartialEq)]
// #[cynic(schema_path = "schema.graphql", query_module = "schema", graphql_type = "Quizzes")]
// pub struct GqQuiz {
//     #[validate(length(min = 1))]
//     pub name: String,
//     pub description: String,
//     pub creator: String,
//     pub image: Option<String>,
// }
//
// #[derive(cynic::QueryFragment, serde::Deserialize, Debug, Clone, PartialEq)]
// #[cynic(schema_path = "schema.graphql", query_module = "schema", graphql_type = "Quizzes")]
// pub struct Quiz {
//     pub quiz_id: u64,
//     pub created_at: DateTime<Utc>,
//
//     pub name: String,
//     pub description: String,
//     pub creator: String,
//     pub image: Option<String>,
// }
//
// #[derive(cynic::QueryFragment, serde::Deserialize, Debug, Clone, PartialEq)]
// #[cynic(schema_path = "schema.graphql", query_module = "schema", graphql_type = "Quizzes")]
// pub struct QuizId {
//     pub quiz_id: u64,
// }
//
// #[derive(cynic::QueryFragment, Debug)]
// #[cynic(schema_path = "schema.graphql", schema_module = "schema", graphql_type = "query_root")]
// struct Quizzes {
//     quizzes: Vec<Quiz>,
// }
//
// #[derive(cynic::FragmentArguments, Debug)]
// struct DeleteQuizArgs {
//     quiz_id: u64,
// }
//
// #[derive(cynic::QueryFragment, Debug)]
// #[cynic(
//     schema_path = "schema.graphql",
//     schema_module = "schema",
//     graphql_type = "mutation_root",
//     argument_struct = "DeleteQuizArgs"
// )]
// struct DeleteQuizzesByPk {
//     #[arguments(quiz_id = &args.quiz_id)]
//     delete_quizzes_by_pk: Option<QuizId>,
// }
//
// #[derive(cynic::FragmentArguments, Debug)]
// struct UpdateQuizArgs {
//     quiz_id: QuizId,
//     draft: DraftQuiz,
// }
//
// // #[derive(cynic::QueryFragment, Debug)]
// // #[cynic(
// //     schema_path = "schema.graphql",
// //     schema_module = "schema",
// //     graphql_type = "mutation_root",
// //     argument_struct = "UpdateQuizArgs"
// // )]
// // struct UpdateQuizzesByPk {
// //     #[arguments(pk_columns = &args.quiz_id)]
// //     #[arguments(_set = &args.draft)]
// //     update_quizzes_by_pk: Option<QuizId>,
// // }
