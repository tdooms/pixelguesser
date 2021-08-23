use super::keys::{GRAPHQL_API, GRAPHQL_SECRET};
use crate::graphql::{Quiz, Round};
use reqwasm::http::{Method, Request};
use serde::de::DeserializeOwned;

#[derive(serde::Deserialize)]
struct Data<T> {
    data: T,
}

#[derive(serde::Deserialize)]
struct QuizzesData {
    pub quizzes: Vec<Quiz>,
}

#[derive(serde::Deserialize)]
struct QuizData {
    pub quiz: Quiz,
    pub rounds: Vec<Round>,
}

pub async fn exec<T: DeserializeOwned>(query: &str) -> Result<T, reqwasm::Error> {
    Request::new(GRAPHQL_API)
        .method(Method::POST)
        .header("content-type", "application/json")
        .header("x-hasura-admin-secret", GRAPHQL_SECRET)
        .body(format!("{{\"query\":\"{}\"}}", query))
        .send()
        .await?
        .json()
        .await
}

pub async fn quizzes() -> Result<Vec<Quiz>, reqwasm::Error> {
    const QUERY: &str =
        "query overview { quizzes { quiz_id name description creator created_at image_url } }";
    let body: Data<QuizzesData> = exec(QUERY).await?;
    Ok(body.data.quizzes)
}

pub async fn quiz() -> Result<(Quiz, Vec<Round>), reqwasm::Error> {
    const QUERY: &str =
        "query overview { quizzes { quiz_id name description creator created_at image_url } }";
    let body: Data<QuizData> = exec(QUERY).await?;
    Ok((body.data.quiz, body.data.rounds))
}
