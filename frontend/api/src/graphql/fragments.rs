use crate::{Quiz, Round};

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
    pub quizzes_by_pk: Option<Quiz>,
    pub rounds: Vec<Round>,
}

#[derive(serde::Deserialize, Debug)]
pub struct CompleteQuizData {
    pub update_quizzes: AffectedRows,
}

#[derive(serde::Deserialize, Debug)]
pub struct CreateQuizData {
    pub insert_quizzes_one: Option<Quiz>,
}

#[derive(serde::Deserialize, Debug)]
pub struct UpdateQuizData {
    pub update_quizzes_one: Option<Quiz>,
}

#[derive(serde::Deserialize, Debug)]
pub struct DeleteQuizData {
    pub delete_quizzes_by_pk: Option<Quiz>,
    pub delete_rounds: AffectedRows,
}

#[derive(serde::Deserialize, Debug)]
pub struct SaveRoundsData {
    pub delete_rounds: AffectedRows,
    pub insert_rounds: AffectedRows,
}
