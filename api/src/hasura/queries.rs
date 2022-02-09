use crate::hasura::GqlQuiz;
use crate::{Quiz, User};
pub use cynic::QueryFragment;

fn base<Req: QueryFragment, Res>(args: Req::Arguments, user: Option<User>) {
    // Serialize the request into graphql json
    let operation = Req::build(args);
    let body = serde_json::to_string(&operation).unwrap();
    log::debug!("Request: {}", body);

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
    let response = builder.json(body).send().await.unwrap();
    let result = operation.decode_response(response.json().await.unwrap()).unwrap();

    log::info!("{:?}", result);
}

pub fn quizzes(user: Option<User>) -> Vec<Quiz> {}

pub fn full_quiz() {}
