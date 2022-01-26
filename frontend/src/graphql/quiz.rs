use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::graphql::ROUND_FIELDS;
use crate::shared::{Error, User};

use super::{exec, AffectedRows, ImageData, Kind, Round};

pub const QUIZ_FIELDS: &str =
    "id public title description explanation image created_at creator {id name}";

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
    id: u64,
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

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Creator {
    pub id: String,
    pub name: String,
}

impl From<User> for Creator {
    fn from(user: User) -> Self {
        Self { id: user.sub, name: user.nickname }
    }
}

#[derive(Validate, Serialize, Default, Debug, Clone, PartialEq)]
pub struct DraftQuiz {
    pub public: bool,

    #[validate(length(min = 1, message = "Quiz must have a name."))]
    pub title: String,
    pub description: String,
    pub explanation: String,
    pub image: Option<ImageData>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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

impl From<Quiz> for DraftQuiz {
    fn from(quiz: Quiz) -> Self {
        Self {
            title: quiz.title,
            public: quiz.public,
            description: quiz.description,
            explanation: quiz.explanation,
            image: quiz.image.map(ImageData::from_url),
        }
    }
}

pub async fn quizzes(user: Option<User>) -> Result<Vec<Quiz>, Error> {
    let str = format!("quizzes {{ {} }}", QUIZ_FIELDS);

    let data: QuizzesData = exec(user, Kind::Query(&str)).await?;
    Ok(data.quizzes)
}

pub async fn quiz(user: Option<User>, quiz_id: u64) -> Result<(Quiz, Vec<Round>), Error> {
    let str = format!(
        "quizzes_by_pk(id: {}) {{ {} }} rounds(where: {{quiz_id: {{ _eq: {} }} }}) {{ {} }}",
        quiz_id, QUIZ_FIELDS, quiz_id, ROUND_FIELDS
    );

    let data: QuizData = exec(user, Kind::Query(&str)).await?;
    Ok((data.quizzes_by_pk, data.rounds))
}

fn serialize(draft: &DraftQuiz) -> String {
    let image = draft.image.as_ref().map(|x| format!(", image: \\\"{}\\\"", x)).unwrap_or_default();
    format!(
        "title:\\\"{}\\\",description:\\\"{}\\\",explanation:\\\"{}\\\"{}",
        draft.title, draft.description, draft.explanation, image
    )
}

pub async fn insert_quiz(
    user: Option<User>,
    mut draft: DraftQuiz,
) -> Result<(Option<u64>, DraftQuiz), Error> {
    if let Some(image) = &mut draft.image {
        image.upload().await?;
    }

    let object = serialize(&draft);
    let str = format!("insert_quizzes_one(object: {{ {} }}) {{ id }}", object);

    let data: CreateQuizData = exec(user, Kind::Mutation(&str)).await?;
    Ok((data.insert_quizzes_one.map(|x| x.id), draft))
}

pub async fn update_quiz(
    user: Option<User>,
    id: u64,
    mut draft: DraftQuiz,
) -> Result<(Option<u64>, DraftQuiz), Error> {
    if let Some(image) = &mut draft.image {
        image.upload().await?;
    }

    let object = serialize(&draft);
    let str = format!(
        "update_quizzes_by_pk(_set: {{ {} }}, pk_columns: {{ id: \\\"{}\\\" }}) {{ id }}",
        object, id
    );
    let data: UpdateQuizData = exec(user, Kind::Mutation(&str)).await?;
    Ok((data.update_quizzes_one.map(|x| x.id), draft))
}

pub async fn delete_quiz(user: Option<User>, quiz_id: u64) -> Result<Option<u64>, Error> {
    let str = format!(
        "delete_rounds(where: {{ id: {{ _eq: \\\"{}\\\" }} }} ) {{affected_rows}} \
    delete_quizzes_by_pk(id: \\\"{}\\\") {{ id }}",
        quiz_id, quiz_id
    );

    let data: DeleteQuizData = exec(user, Kind::Mutation(&str)).await?;
    Ok(data.delete_quizzes_by_pk.map(|x| x.id))
}
