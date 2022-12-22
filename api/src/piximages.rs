use piximages::Response;
use reqwest::Client;

use crate::{Error, IMAGE_ENDPOINT};

pub async fn upload(client: &Client, token: String, body: String) -> Result<Response, Error> {
    let url = format!("{IMAGE_ENDPOINT}/upload");

    client
        .post(&url)
        .header("Authorization", token)
        .body(body)
        .send()
        .await
        .map_err(|_| Error::Unreachable("piximages", url))?
        .error_for_status()
        .map_err(|_| Error::ErrorStatus("piximages"))?
        .json()
        .await
        .map_err(|e| Error::InvalidResponse("piximages", e.to_string()))
}
