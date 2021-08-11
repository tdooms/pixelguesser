use std::fmt::Display;

use serde::de::DeserializeOwned;

use crate::constants::{SESSION_ENDPOINT, IMAGE_ENDPOINT};

pub async fn session_get<Call, Ret, Msg>(uri: impl Display, callback: Call) -> Msg
    where Ret: DeserializeOwned,
          Call: FnOnce(Ret) -> Msg
{
    let response = reqwest::get(format!("{}/{}", SESSION_ENDPOINT, uri))
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    callback(response)
}

pub fn image_url<T: Display>(image: Option<T>) -> String {
    match image {
        None => format!("{}/empty.jpg", IMAGE_ENDPOINT),
        Some(image) => format!("{}/{}", IMAGE_ENDPOINT, image)
    }
}