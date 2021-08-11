use std::fmt::Display;

use serde::de::DeserializeOwned;
use serde::Serialize;

use reqwest::Method;

use crate::constants::{IMAGE_ENDPOINT, SESSION_ENDPOINT};

async fn sessions_api_call<C, R, M>(uri: impl Display, method: Method, body: &impl Serialize, callback: C) -> M
    where R: DeserializeOwned, C: FnOnce(R) -> M
{
    let url = format!("{}/{}", SESSION_ENDPOINT, uri);
    let client = reqwest::Client::new();
    let result = client.request(method, url).json(body).send().await.unwrap().json().await.unwrap();
    callback(result)
}

pub async fn check_session<C, R, M>(session_id: u64, callback: C) -> M
    where R: DeserializeOwned, C: FnOnce(R) -> M
{
    sessions_api_call(format!("{}/check", session_id), Method::GET, &(), callback)
}

pub async fn create_session<C, R, M>(callback: C) -> M
    where R: DeserializeOwned, C: FnOnce(R) -> M
{
    sessions_api_call(format!("create"), Method::POST, &(), callback)
}


pub fn image_url<T: Display>(image: Option<T>) -> String {
    match image {
        None => format!("{}/empty.jpg", IMAGE_ENDPOINT),
        Some(image) => format!("{}/{}", IMAGE_ENDPOINT, image)
    }
}