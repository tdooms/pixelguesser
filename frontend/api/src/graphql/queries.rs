use std::fmt::Debug;

use hasura::*;
use reqwasm::http::{Method, Request};
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::{DraftQuiz, DraftRound, Error, FullQuiz, Quiz, QuizPk, Round, User, GRAPHQL_ENDPOINT};

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
enum Response<T> {
    Data { data: T },
    Errors { errors: Vec<GraphqlError> },
}

#[derive(serde::Deserialize, Debug)]
pub struct GraphqlError {
    pub extensions: Value,
    pub message: String,
}

pub async fn exec<T: DeserializeOwned + Debug>(
    user: Option<User>,
    body: String,
) -> Result<T, Error> {
    log::debug!("request {}", body);

    let builder = Request::new(GRAPHQL_ENDPOINT)
        .method(Method::POST)
        .header("content-type", "application/json");

    let builder = match &user {
        Some(user) => builder.header("authorization", &user.token),
        None => builder,
    };

    let response = builder.body(body).send().await?.text().await?;

    log::debug!("response {}", response);

    match serde_json::from_str(&response)? {
        Response::Data { data } => Ok(data),
        Response::Errors { errors } => {
            Err(Error::Graphql(errors.into_iter().map(|x| x.message).collect()))
        }
    }
}

pub async fn quizzes(user: Option<User>) -> Result<Vec<Quiz>, Error> {
    let body: Query<Quiz> = QueryBuilder::default().returning(Quiz::all()).build().unwrap();
    exec(user, query!(body)).await
}

pub async fn search_quizzes(user: Option<User>, query: String) -> Result<Vec<Quiz>, Error> {
    let value = format!("\\\"%{}%\\\"", query);
    let condition = Condition { op: "_ilike", value: value.as_str() };
    let conditions = Conditions::Field(Quiz::title(), vec![condition]);

    let body = QueryBuilder::default()
        .conditions(vec![conditions])
        .returning(Quiz::all())
        .build()
        .unwrap();

    exec(user, query!(body)).await
}

pub async fn full_quiz(user: Option<User>, id: u64) -> Result<FullQuiz, Error> {
    let value = id.to_string();
    let condition = Condition { op: "_eq", value: value.as_str() };
    let conditions = Conditions::Field(Round::quiz_id(), vec![condition]);

    let quiz =
        QueryByPkBuilder::default().pk(QuizPk { id }).returning(Quiz::all()).build().unwrap();

    let rounds: Query<Round> = QueryBuilder::default()
        .conditions(vec![conditions])
        .returning(Round::all())
        .build()
        .unwrap();

    exec(user, query!(quiz, rounds)).await
}

pub async fn create_quiz(user: User, draft: DraftQuiz) -> Result<Option<Quiz>, Error> {
    let body = InsertOneBuilder::default().returning(Quiz::all()).object(draft).build().unwrap();

    exec(Some(user), mutation!(body)).await
}

pub async fn update_quiz(user: User, id: u64, draft: DraftQuiz) -> Result<Option<Quiz>, Error> {
    let body = UpdateByPkBuilder::default()
        .pk(QuizPk { id })
        .returning(Quiz::all())
        .set(draft)
        .build()
        .unwrap();

    exec(Some(user), mutation!(body)).await
}

pub async fn delete_quiz(user: User, id: u64) -> Result<Option<Quiz>, Error> {
    let body =
        DeleteByPkBuilder::default().pk(QuizPk { id }).returning(Quiz::all()).build().unwrap();

    exec(Some(user), mutation!(body)).await
}

pub async fn save_rounds(
    user: User,
    quiz_id: u64,
    rounds: Vec<DraftRound>,
) -> Result<Vec<Round>, Error> {
    let value = quiz_id.to_string();
    let condition = Condition { op: "_eq", value: value.as_str() };
    let conditions = Conditions::Field(Round::quiz_id(), vec![condition]);

    let delete = DeleteBuilder::default()
        .returning(Round::all())
        .conditions(vec![conditions])
        .build()
        .unwrap();

    let insert = InsertBuilder::default().returning(Round::all()).objects(rounds).build().unwrap();

    exec(Some(user), mutation!(delete, insert)).await
}
