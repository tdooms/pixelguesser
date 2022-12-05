use piximages::Response;
use reqwest::Client;

use crate::{Error, UPLOAD_ENDPOINT};

pub async fn upload(client: &Client, token: String, body: String) -> Result<Response, Error> {
    let url = format!("{UPLOAD_ENDPOINT}/upload");

    client
        .post(&url)
        .header("Authorization", token)
        .body(body)
        .send()
        .await
        .map_err(|_| Error::UnreachableHost("piximages", url))?
        .error_for_status()
        .map_err(|_| Error::StatusCode("piximages"))?
        .json()
        .await
        .map_err(|_| Error::InvalidResponse("piximages"))
}
