use std::fs::File;

use reqwest::Client;

use api::{Credentials, Image, Quiz, Tokens, User, AUTH_ENDPOINT};

use crate::auth::{delete_user, upload_user};
use crate::graphql::{delete_quizzes, upload_quizzes};
use crate::images::{delete_images, upload_images};

mod auth;
mod graphql;
mod images;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Quizzes {
    quizzes: Vec<Quiz>,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenv::dotenv().unwrap();

    let email = std::env::var("ADMIN_EMAIL").unwrap();
    let password = std::env::var("ADMIN_PASSWORD").unwrap();
    let credentials = Credentials { email: email.clone(), password };

    let tokens: Tokens = Client::new()
        .post(format!("{AUTH_ENDPOINT}/login"))
        .json(&credentials)
        .send()
        .await
        .map_err(|_| "Cannot connect to the auth server, please verify it is running")
        .unwrap()
        .json()
        .await
        .unwrap();

    let user = User {
        user_id: Some(tokens.id.parse().unwrap()),
        nickname: "admin".to_string(),
        image: Image::default(),
        email,
        last_seen: None,
        verified: true,
    };

    let file = File::open("init/create.json").unwrap();
    let Quizzes { mut quizzes } = serde_json::from_reader(file).unwrap();

    let bearer = format!("Bearer {}", tokens.bearer);

    log::info!("deleted quizzes: {:?}", delete_quizzes(bearer.clone()).await);
    log::info!("deleted images: {:?}", delete_images(bearer.clone()).await);
    log::info!("deleted user: {:?}", delete_user(bearer.clone()).await);

    upload_user(user, bearer.clone()).await;
    upload_images(&mut quizzes, bearer.clone()).await;
    upload_quizzes(&mut quizzes, tokens.id.parse().unwrap(), bearer).await;
}
