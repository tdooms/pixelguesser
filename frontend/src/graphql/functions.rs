use std::fmt::Debug;

use reqwasm::http::{Method, Request};
use serde::de::DeserializeOwned;

use crate::error::Error;
use crate::graphql::{DraftQuiz, DraftRound, Quiz, Round};
use crate::graphql::convert::*;
use crate::graphql::data::*;

use super::keys::{GRAPHQL_API, GRAPHQL_SECRET};

const QUIZ_FIELDS: &str = "quiz_id name description creator created_at image_url";
const ROUND_FIELDS: &str = "round_id quiz_id index answer points guesses speed image_url";

pub enum Kind<'r> {
    Query(&'r str),
    Mutation(&'r str),
    Subscription(&'r str),
}

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
enum Response<T> {
    Data { data: T },
    Errors { errors: Vec<GraphqlError> },
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

pub async fn quizzes() -> Result<Vec<Quiz>, Error> {
    let str = format!("quizzes {{ {} }}", QUIZ_FIELDS);

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
        image_url: None,
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
