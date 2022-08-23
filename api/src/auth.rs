use crate::{Error, AUTH_ENDPOINT};
use pixauth::{Credentials, Tokens};
use reqwest::Client;
use std::rc::Rc;

pub async fn login(cred: Rc<Credentials>) -> Result<Tokens, Error> {
    let url = format!("{AUTH_ENDPOINT}/login");
    Ok(Client::new().post(url).json(&cred).send().await?.json().await?)
}

pub async fn signup(cred: Rc<Credentials>) -> Result<Tokens, Error> {
    let url = format!("{AUTH_ENDPOINT}/signup");
    Ok(Client::new().post(url).json(&cred).send().await?.json().await?)
}
