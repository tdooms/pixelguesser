use super::{exec, AffectedRows, Kind, Round};
use crate::error::Error;
use crate::graphql::ROUND_FIELDS;
use crate::structs::ImageData;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

pub const QUIZ_FIELDS: &str = "quiz_id name description creator created_at image_url";

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
pub struct CompleteQuizData {
    pub update_quizzes: AffectedRows,
}

#[derive(serde::Deserialize, Debug)]
pub struct CreateQuizData {
    pub insert_quizzes_one: AffectedRows,
}

#[derive(Validate, Serialize, Debug, Default, Clone, PartialEq)]
pub struct DraftQuiz {
    #[validate(length(min = 1))]
    pub name: String,
    pub description: String,
    pub creator: String,
    pub image: Option<ImageData>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Quiz {
    pub quiz_id: u64,
    pub created_at: DateTime<Utc>,

    pub name: String,
    pub description: String,
    pub creator: String,
    pub image: Option<String>,
}

impl From<Quiz> for DraftQuiz {
    fn from(quiz: Quiz) -> Self {
        Self {
            name: quiz.name,
            description: quiz.description,
            creator: quiz.creator,
            image: quiz.image.map(ImageData::from_url),
        }
    }
}

pub async fn quizzes() -> Result<Vec<Quiz>, Error> {
    let str = format!("quizzes {{ {} }}", QUIZ_FIELDS);

    let data: QuizzesData = exec(Kind::Query(&str)).await?;
    Ok(data.quizzes)
}

pub async fn quiz(quiz_id: u64) -> Result<(Quiz, Vec<Round>), Error> {
    let str = format!(
        "quizzes_by_pk(quiz_id: {}) {{ {} }} rounds(where: {{quiz_id: {{ _eq: {} }} }}) {{ {} }}",
        quiz_id, QUIZ_FIELDS, quiz_id, ROUND_FIELDS
    );

    let data: QuizData = exec(Kind::Query(&str)).await?;
    Ok((data.quizzes_by_pk, data.rounds))
}

pub async fn create_quiz(draft: DraftQuiz) -> Result<u64, Error> {
    if let Some(image) = &draft.image {
        image.upload().await?;
    }

    let object = serde_json::to_string(&draft).unwrap();
    let str = format!("insert_quizzes_one(object: {{ {} }})", object);

    let data: CreateQuizData = exec(Kind::Mutation(&str)).await?;
    Ok(data.insert_quizzes_one.affected_rows)
}
