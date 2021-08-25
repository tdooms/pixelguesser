use std::fmt::Debug;

use reqwasm::http::{Method, Request};
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::error::Error;
use crate::graphql::{Quiz, Round, DraftRound, DraftQuiz};

use super::keys::{GRAPHQL_API, GRAPHQL_SECRET};
use crate::graphql::convert::SerRound;

#[derive(serde::Deserialize, Debug)]
struct GraphqlError {
    extensions: Value,
    message: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
enum Response<T> {
    Data { data: T },
    Errors { errors: Vec<GraphqlError> },
}

#[derive(serde::Deserialize, Debug)]
struct AffectedRows {
    affected_rows: u64
}

#[derive(serde::Deserialize, Debug)]
struct QuizzesData {
    pub quizzes: Vec<Quiz>,
}

#[derive(serde::Deserialize, Debug)]
struct QuizData {
    pub quizzes_by_pk: Quiz,
    pub rounds: Vec<Round>,
}

#[derive(serde::Deserialize, Debug)]
struct SaveRoundsData {
    pub delete_rounds: AffectedRows,
    pub insert_rounds: AffectedRows
}

#[derive(serde::Deserialize, Debug)]
struct CompleteQuizData {
    pub update_quizzes: AffectedRows,
}

#[derive(serde::Deserialize, Debug)]
struct CreateQuizData {
    pub insert_quizzes_one: AffectedRows,
}

pub enum Kind<'r> {
    Query(&'r str),
    Mutation(&'r str),
    Subscription(&'r str)
}

pub async fn exec<T: DeserializeOwned + Debug>(query: Kind<'_>) -> Result<T, Error> {
    let body = match query {
        Kind::Query(str) => format!("{{\"query\":\"query {{ {} }}\"}}", str),
        Kind::Mutation(str) => format!("{{\"query\":\"mutation {{ {} }}\"}}", str),
        Kind::Subscription(str) => format!("{{\"query\":\"subscription {{ {} }}\"}}", str),
    };

    let response: Response<T> = Request::new(GRAPHQL_API)
        .method(Method::POST)
        .header("content-type", "application/json")
        .header("x-hasura-admin-secret", GRAPHQL_SECRET)
        .body(body)
        .send()
        .await?
        .json()
        .await?;

    log::info!("{:?}", response);

    match response {
        Response::Data { data } => Ok(data),
        Response::Errors { errors } => Err(Error::Graphql(errors.into_iter().map(|x| x.message).collect()))
    }
}

const QUIZ_FIELDS: &str = "quiz_id name description creator created_at image_url";
const ROUND_FIELDS: &str = "answer points guesses image_url";

pub async fn quizzes() -> Result<Vec<Quiz>, Error> {
    let str= format!("quizzes {{ {} }}", QUIZ_FIELDS);

    let data: QuizzesData = exec(Kind::Query(&str)).await?;
    Ok(data.quizzes)
}

pub async fn quiz(quiz_id: u64) -> Result<(Quiz, Vec<Round>), Error> {
    let str = format!("quizzes_by_pk(quiz_id: {}) {{ {} }} rounds(where: {{quiz_id: {{ _eq: {} }} }}) {{ {} }}", quiz_id, QUIZ_FIELDS, quiz_id, ROUND_FIELDS);

    let data: QuizData = exec(Kind::Query(&str)).await?;
    Ok((data.quizzes_by_pk, data.rounds))
}

pub async fn save_rounds(quiz_id: u64, rounds: &[DraftRound]) -> Result<(u64, u64), Error> {
    let ser: Vec<_> = rounds.iter().enumerate().map(|(index, draft)| SerRound {
        quiz_id,
        index: index as u64,
        info: draft.info.clone(),
        options: draft.options.clone(),
        image_url: None
    }).collect();

    let objects = serde_json::to_string(&ser).unwrap();
    let str = format!("delete_rounds(where: {{ quiz_id: (_eq: {}) }} ) insert_rounds(objects: {}) {{ affected_rows }}", quiz_id, objects);

    let data: SaveRoundsData = exec(Kind::Mutation(&str)).await?;
    Ok((data.delete_rounds.affected_rows, data.insert_rounds.affected_rows))
}

pub async fn create_quiz(draft: DraftQuiz) -> Result<u64, Error> {
    let object = serde_json::to_string(&draft).unwrap();
    let str = format!("insert_quizzes_one(object: {{ {} }})", object);

    let data: CreateQuizData = exec(Kind::Mutation(&str)).await?;
    Ok(data.insert_quizzes_one.affected_rows)
}

pub async fn complete_quiz(quiz_id: u64) -> Result<u64, Error> {
    let str = format!("update_quizzes(where: {{ quiz_id: {{ _eq: {} }} }}, _set: {{ draft: false}} ) {{ affected_rows }}", quiz_id);
    let data: CompleteQuizData = exec(Kind::Mutation(&str)).await?;
    Ok(data.update_quizzes.affected_rows)
}
