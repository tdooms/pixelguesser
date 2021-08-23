use std::fmt::Debug;

use reqwasm::http::{Method, Request};
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::graphql::{Quiz, Round};

use super::keys::{GRAPHQL_API, GRAPHQL_SECRET};
use crate::error::Error;

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
struct QuizzesData {
    pub quizzes: Vec<Quiz>,
}

#[derive(serde::Deserialize, Debug)]
struct QuizData {
    pub quizzes_by_pk: Quiz,
    pub rounds: Vec<Round>,
}

pub async fn exec<T: DeserializeOwned + Debug>(query: &str) -> Result<T, Error> {
    let response: Response<T> = Request::new(GRAPHQL_API)
        .method(Method::POST)
        .header("content-type", "application/json")
        .header("x-hasura-admin-secret", GRAPHQL_SECRET)
        .body(format!("{{\"query\":\"{}\"}}", query))
        .send()
        .await?
        .json()
        .await?;

    log::info!("{:?}", response);

    match response {
        Response::Data {data} => Ok(data),
        Response::Errors {errors} => Err(Error::Graphql(errors.into_iter().map(|x| x.message).collect()))
    }
}

pub async fn quizzes() -> Result<Vec<Quiz>, Error> {
    const QUERY: &str =
        "query overview { quizzes { quiz_id name description creator created_at image_url } }";
    let data: QuizzesData = exec(QUERY).await?;
    Ok(data.quizzes)
}

pub async fn quiz(quiz_id: u64) -> Result<(Quiz, Vec<Round>), Error> {
    let query = format!("query {{ quizzes_by_pk(quiz_id: {}) {{ quiz_id name description creator created_at image_url }} rounds(where: {{quiz_id: {{ _eq: {} }} }}) {{ answer points guesses image_url }} }}", quiz_id, quiz_id);
    let data: QuizData = exec(&query).await?;
    Ok((data.quizzes_by_pk, data.rounds))
}
