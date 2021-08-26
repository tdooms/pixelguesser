use serde_json::Value;

use crate::graphql::{Quiz, Round};

#[derive(serde::Deserialize, Debug)]
pub struct GraphqlError {
    pub extensions: Value,
    pub message: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct AffectedRows {
    pub affected_rows: u64,
}

#[derive(serde::Deserialize, Debug)]
pub struct QuizzesData {
    pub quizzes: Vec<Quiz>,
}

#[derive(serde::Deserialize, Debug)]
pub struct QuizData {
    pub quizzes_by_pk: Quiz,
    pub rounds: Vec<Round>,
}

#[derive(serde::Deserialize, Debug)]
pub struct SaveRoundsData {
    pub delete_rounds: AffectedRows,
    pub insert_rounds: AffectedRows,
}

#[derive(serde::Deserialize, Debug)]
pub struct CompleteQuizData {
    pub update_quizzes: AffectedRows,
}

#[derive(serde::Deserialize, Debug)]
pub struct CreateQuizData {
    pub insert_quizzes_one: AffectedRows,
}