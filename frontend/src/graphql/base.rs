use crate::shared::{Error, GRAPHQL_ENDPOINT};
use crate::Auth;
use reqwasm::http::{Method, Request};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::fmt::Debug;

#[allow(dead_code)]
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

#[derive(serde::Deserialize, Debug)]
pub struct GraphqlError {
    pub extensions: Value,
    pub message: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct AffectedRows {
    pub affected_rows: u64,
}

pub async fn exec<T: DeserializeOwned + Debug>(user: Auth, query: Kind<'_>) -> Result<T, Error> {
    let body = match query {
        Kind::Query(str) => format!("{{\"query\":\"query {{ {} }}\"}}", str),
        Kind::Mutation(str) => {
            format!("{{\"query\":\"mutation {{ {} }}\" }}", str)
        }
        Kind::Subscription(str) => format!("{{\"query\":\"subscription {{ {} }}\"}}", str),
    };

    log::debug!("Request: {}", body);

    let builder = Request::new(GRAPHQL_ENDPOINT)
        .method(Method::POST)
        .header("content-type", "application/json");

    let builder = match &user {
        Auth::User(user) => builder.header("authorization", &user.token),
        _ => builder,
    };

    let response: Response<T> = builder.body(body).send().await?.json().await?;

    match response {
        Response::Data { data } => {
            log::debug!("Response: {:?}", data);
            Ok(data)
        }
        Response::Errors { errors } => {
            log::warn!("{:?}", errors);
            Err(Error::Graphql(errors.into_iter().map(|x| x.message).collect()))
        }
    }
}
