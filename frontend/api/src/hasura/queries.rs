use crate::error::Error;
use crate::{DraftQuiz, DraftRound, FullQuiz, Quiz, User};
use cynic::QueryFragment;
use cynic::{serde_json, MutationBuilder, QueryBuilder};
use keys::GRAPHQL_ENDPOINT;
use reqwasm::http::{Method, Request};

async fn base<'a, Req: 'a + QueryBuilder<'a>, Res>(
    args: Req::Arguments,
    user: Option<User>,
) -> Result<(), Error> {
    // Serialize the request into graphql json
    let operation = Req::build(args);
    let body = serde_json::to_string(&operation)?;

    // Start building the request with the correct headers
    let builder = Request::new(GRAPHQL_ENDPOINT)
        .method(Method::POST)
        .header("content-type", "application/json");

    // Add authentication when available
    let builder = match &user {
        Some(user) => builder.header("authorization", &user.token),
        None => builder,
    };

    // send request, await response and decode
    let response = builder.body(body).send().await?;
    let result = operation.decode_response(response.json().await?)?;

    Ok(())
}

pub async fn quizzes(user: Option<User>) -> Result<Vec<Quiz>, Error> {
    todo!()
}

pub async fn create_quiz(user: Option<User>, quiz: DraftQuiz) -> Result<Quiz, Error> {
    todo!()
}

pub async fn update_quiz(user: Option<User>, id: u64, quiz: DraftQuiz) -> Result<Quiz, Error> {
    todo!()
}

pub async fn delete_quiz(user: Option<User>, id: u64) -> Result<(), Error> {
    todo!()
}

pub async fn full_quiz(user: Option<User>, id: u64) -> Result<FullQuiz, Error> {
    todo!()
}

pub async fn save_rounds(
    user: Option<User>,
    id: u64,
    rounds: Vec<DraftRound>,
) -> Result<(), Error> {
    todo!()
}
