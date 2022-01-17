use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::error::Error;
use crate::graphql::ROUND_FIELDS;

use super::{exec, AffectedRows, ImageData, Kind, Round};

pub const QUIZ_FIELDS: &str = "quiz_id name description creator created_at image";

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
pub struct QuizId {
    quiz_id: u64,
}

#[derive(serde::Deserialize, Debug)]
pub struct CompleteQuizData {
    pub update_quizzes: AffectedRows,
}

#[derive(serde::Deserialize, Debug)]
pub struct CreateQuizData {
    pub insert_quizzes_one: Option<QuizId>,
}

#[derive(serde::Deserialize, Debug)]
pub struct UpdateQuizData {
    pub update_quizzes_one: Option<QuizId>,
}

#[derive(serde::Deserialize, Debug)]
pub struct DeleteQuizData {
    pub delete_quizzes_by_pk: Option<QuizId>,
    pub delete_rounds: AffectedRows,
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

fn serialize(draft: &DraftQuiz) -> String {
    let image = draft.image.as_ref().map(|x| format!(", image: \\\"{}\\\"", x)).unwrap_or_default();
    format!(
        "name:\\\"{}\\\",description:\\\"{}\\\",creator:\\\"{}\\\"{}",
        draft.name, draft.description, draft.creator, image
    )
}

pub async fn insert_quiz(mut draft: DraftQuiz) -> Result<(Option<u64>, DraftQuiz), Error> {
    if let Some(image) = &mut draft.image {
        image.upload().await?;
    }

    let object = serialize(&draft);
    let str = format!("insert_quizzes_one(object: {{ {} }}) {{ quiz_id }}", object);

    let data: CreateQuizData = exec(Kind::Mutation(&str)).await?;
    Ok((data.insert_quizzes_one.map(|x| x.quiz_id), draft))
}

pub async fn update_quiz(id: u64, mut draft: DraftQuiz) -> Result<(Option<u64>, DraftQuiz), Error> {
    if let Some(image) = &mut draft.image {
        image.upload().await?;
    }

    let object = serialize(&draft);
    let str = format!(
        "update_quizzes_by_pk(_set: {{ {} }}, pk_columns: {{ quiz_id: \\\"{}\\\" }}) {{ quiz_id }}",
        object, id
    );
    let data: UpdateQuizData = exec(Kind::Mutation(&str)).await?;
    Ok((data.update_quizzes_one.map(|x| x.quiz_id), draft))
}

pub async fn delete_quiz(quiz_id: u64) -> Result<Option<u64>, Error> {
    let str = format!(
        "delete_rounds(where: {{ quiz_id: {{ _eq: \\\"{}\\\" }} }} ) {{affected_rows}} \
    delete_quizzes_by_pk(quiz_id: \\\"{}\\\") {{ quiz_id }}",
        quiz_id, quiz_id
    );

    let data: DeleteQuizData = exec(Kind::Mutation(&str)).await?;
    Ok(data.delete_quizzes_by_pk.map(|x| x.quiz_id))
}
