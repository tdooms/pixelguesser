use crate::keys::{GRAPHQL_API, GRAPHQL_SECRET};
use serde::de::DeserializeOwned;
use reqwasm::http::{Method, Request};

pub async fn exec<T: DeserializeOwned>(query: &str) -> Result<T, reqwasm::Error> {
    Request::new(GRAPHQL_API)
        .method(Method::GET)
        .header("content-type", "application/json")
        .header("x-hasura-admin-secret", GRAPHQL_SECRET)
        .body(format!("\"query\":\"{}\"", query))
        .send()
        .await?
        .json()
        .await
}

pub async fn quizzes<T: DeserializeOwned>() -> Result<T, reqwasm::Error> {
    const QUERY: &str = "query overview { quizzes { data { name description author created image_url} } }";
    exec(QUERY).await
}