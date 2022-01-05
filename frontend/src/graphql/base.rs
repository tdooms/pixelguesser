use crate::constants::{GRAPHQL_ENDPOINT, HASURA_SECRET};
use crate::error::Error;
use crate::graphql::data::GraphqlError;
use reqwasm::http::Method;
use serde::de::DeserializeOwned;
use serde_json::Value;
use sessions::Request;
use std::fmt::Debug;

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

pub async fn exec<T: DeserializeOwned + Debug>(query: Kind<'_>) -> Result<T, Error> {
    let body = match query {
        Kind::Query(str) => format!("{{\"query\":\"query {{ {} }}\"}}", str),
        Kind::Mutation(str) => format!("{{\"query\":\"mutation {{ {} }}\"}}", str),
        Kind::Subscription(str) => format!("{{\"query\":\"subscription {{ {} }}\"}}", str),
    };

    let response: Response<T> = Request::new(GRAPHQL_ENDPOINT)
        .method(Method::POST)
        .header("content-type", "application/json")
        .header("x-hasura-admin-secret", HASURA_SECRET)
        .body(body)
        .send()
        .await?
        .json()
        .await?;

    match response {
        Response::Data { data } => {
            log::info!("{:?}", data);
            Ok(data)
        }
        Response::Errors { errors } => {
            log::warn!("{:?}", data);
            Err(Error::Graphql(errors.into_iter().map(|x| x.message).collect()))
        }
    }
}
