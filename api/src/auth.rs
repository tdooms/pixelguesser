use std::rc::Rc;

use reqwest::Client;

use pixauth::Tokens;

use crate::{Credentials, Error, AUTH_ENDPOINT};

pub async fn login(cred: Rc<Credentials>) -> Result<Tokens, Error> {
    let url = format!("{AUTH_ENDPOINT}/login");
    Ok(Client::new().post(url).json(&cred).send().await?.json().await?)
}

pub async fn signup(cred: Rc<Credentials>) -> Result<Tokens, Error> {
    let url = format!("{AUTH_ENDPOINT}/signup");
    Ok(Client::new().post(url).json(&cred).send().await?.json().await?)
}
