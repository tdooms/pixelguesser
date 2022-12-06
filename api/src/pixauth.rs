use std::rc::Rc;

use reqwest::Client;

use pixauth::Tokens;

use crate::{Credentials, Error, AUTH_ENDPOINT};

async fn send(client: &Client, url: String, cred: Rc<Credentials>) -> Result<Tokens, Error> {
    client
        .post(&url)
        .json(&cred)
        .send()
        .await
        .map_err(|_| Error::Unreachable("pixauth", url))?
        .error_for_status()
        .map_err(|_| Error::ErrorStatus("pixauth"))?
        .json()
        .await
        .map_err(|e| Error::InvalidResponse("pixauth", e.to_string()))
}

pub async fn login(cred: Rc<Credentials>) -> Result<Tokens, Error> {
    send(&Client::new(), format!("{AUTH_ENDPOINT}/login"), cred).await
}

pub async fn signup(cred: Rc<Credentials>) -> Result<Tokens, Error> {
    send(&Client::new(), format!("{AUTH_ENDPOINT}/signup"), cred).await
}
