use cynic::{serde_json, MutationBuilder, Operation, QueryBuilder};
use reqwasm::http::{Method, Request};
use std::future::Future;
use std::pin::Pin;

use keys::GRAPHQL_ENDPOINT;

use crate::error::Error;
use crate::{
    DraftQuiz, DraftRound, FullQuiz, InsertQuizzesOne, InsertQuizzesOneArgs, Quiz, Quizzes,
    QuizzesPkColumnsInput, UpdateQuizArgs, UpdateQuizzesByPk, User,
};

async fn base<'a, T: 'a>(operation: Operation<'a, T>, user: Option<User>) -> Result<T, Error> {
    // Serialize the request into graphql json
    let body = serde_json::to_string(&operation)?;

    log::info!("graphql: {}", body);

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
}

fn query<'a, Req: 'a + QueryBuilder<'a>>(
    args: Req::Arguments,
    user: Option<User>,
) -> Pin<Box<dyn Future<Output = Result<Req::ResponseData, Error>> + 'a>> {
    Box::pin(base(Req::build(args), user))
}

fn mutation<'a, Req: 'a + MutationBuilder<'a>>(
    args: Req::Arguments,
    user: Option<User>,
) -> Pin<Box<dyn Future<Output = Result<Req::ResponseData, Error>> + 'a>> {
    Box::pin(base(Req::build(args), user))
}

pub async fn quizzes(user: Option<User>) -> Result<Vec<Quiz>, Error> {
    Ok(query::<Quizzes>((), user).await?.quizzes)
}

pub async fn create_quiz(user: Option<User>, draft: DraftQuiz) -> Result<Option<Quiz>, Error> {
    Ok(mutation::<InsertQuizzesOne>(InsertQuizzesOneArgs { draft: draft.into() }, user)
        .await?
        .insert_quizzes_one)
    // Err(Error::Empty)
}

pub async fn update_quiz(
    user: Option<User>,
    id: u64,
    draft: DraftQuiz,
) -> Result<Option<Quiz>, Error> {
    let args = UpdateQuizArgs { quiz_id: QuizzesPkColumnsInput { id }, draft: Some(draft.into()) };
    Ok(mutation::<UpdateQuizzesByPk>(args, user).await?.update_quizzes_by_pk)
}

pub async fn delete_quiz(_user: Option<User>, _id: u64) -> Result<(), Error> {
    Err(Error::Empty)
}

pub async fn full_quiz(_user: Option<User>, _id: u64) -> Result<FullQuiz, Error> {
    Err(Error::Empty)
}

pub async fn save_rounds(
    _user: Option<User>,
    _id: u64,
    _rounds: Vec<DraftRound>,
) -> Result<(), Error> {
    Err(Error::Empty)
}
