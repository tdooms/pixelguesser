use crate::Error;
use reqwest::Client;

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Default)]
pub struct User {
    pub nickname: String,
    pub name: String,
    pub picture: String,
    pub updated_at: String,
    pub email: String,
    pub email_verified: bool,
}

#[derive(serde::Serialize)]
pub struct Login {
    pub password: String,
    pub email: String,
}

#[derive(serde::Serialize)]
pub struct Signup {
    pub password: String,
    pub email: String,
}

#[derive(serde::Deserialize)]
struct Response {
    #[serde(flatten)]
    user: User,
    token: String,
}

pub async fn login(cred: Login) -> Result<(User, String), Error> {
    let url = "localhost:8904/login";
    let response: Response = Client::new().post(url).json(&cred).send().await?.json().await?;

    Ok((response.user, response.token))
}

pub async fn signup(cred: Signup) -> Result<(User, String), Error> {
    let url = "localhost:8904/signup";
    let response: Response = Client::new().post(url).json(&cred).send().await?.json().await?;

    Ok((response.user, response.token))
}
