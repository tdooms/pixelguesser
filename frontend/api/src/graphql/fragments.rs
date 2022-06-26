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
pub struct FullQuizData {
    pub quizzes_by_pk: Option<Quiz>,
    pub rounds: Vec<Round>,
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
    pub delete_rounds: Vec<Round>,
}

#[derive(serde::Deserialize, Debug)]
pub struct RoundReturn {
    pub returning: Vec<Round>,
}

#[derive(serde::Deserialize, Debug)]
pub struct SaveRoundsData {
    pub delete_rounds: RoundReturn,
    pub insert_rounds: RoundReturn,
}
