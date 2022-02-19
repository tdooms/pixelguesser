use cynic::{serde_json, GraphQlResponse, MutationBuilder, QueryBuilder};
use reqwasm::http::{Method, Request};
use std::future::Future;
use std::pin::Pin;

use keys::GRAPHQL_ENDPOINT;

use crate::error::Error;
use crate::hasura::schema::Quizzes;
use crate::{DraftQuiz, DraftRound, FullQuiz, Quiz, User};

fn base<'a, Req: 'a + QueryBuilder<'a>>(
    args: Req::Arguments,
    user: Option<User>,
) -> Pin<Box<dyn Future<Output = Result<Req::ResponseData, Error>> + 'a>> {
    let fut = async move {
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
        operation.decode_response(response.json().await?)?.data.ok_or(Error::Empty)
    };

    Box::pin(fut)
}

pub async fn quizzes(user: Option<User>) -> Result<Vec<Quiz>, Error> {
    // Ok(base::<Quizzes>((), user).await?.quizzes)
    Err(Error::Empty)
}

pub async fn create_quiz(_user: Option<User>, _quiz: DraftQuiz) -> Result<Quiz, Error> {
    todo!()
}

pub async fn update_quiz(_user: Option<User>, _id: u64, _quiz: DraftQuiz) -> Result<Quiz, Error> {
    todo!()
}

pub async fn delete_quiz(_user: Option<User>, _id: u64) -> Result<(), Error> {
    todo!()
}

pub async fn full_quiz(_user: Option<User>, _id: u64) -> Result<FullQuiz, Error> {
    todo!()
}

pub async fn save_rounds(
    _user: Option<User>,
    _id: u64,
    _rounds: Vec<DraftRound>,
) -> Result<(), Error> {
    todo!()
}
