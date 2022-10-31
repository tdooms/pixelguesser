mod hasura;
mod images;

use std::fs::File;

use hasura::{mutation, Delete, Insert, InsertOne};
use reqwest::Client;

use api::{Credentials, Image, Quiz, Tokens, User, AUTH_ENDPOINT, GRAPHQL_ENDPOINT};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Quizzes {
    quizzes: Vec<Quiz>,
}

async fn create_quizzes(bearer: String, creator_id: String) {
    let file = File::open("init/create.json").unwrap();
    let Quizzes { mut quizzes } = serde_json::from_reader(file).unwrap();

    for quiz in &mut quizzes {
        quiz.creator_id = Some(creator_id.clone());
        convert_image(&mut quiz.image, bearer.clone()).await;

        for (index, round) in &mut quiz.rounds.iter_mut().enumerate() {
            convert_image(&mut round.image, bearer.clone()).await;
            round.index = index as u64
        }
    }
}

async fn create_user(bearer: String) {
    let user = User {
        id: tokens.id.clone(),
        nickname: "admin".to_string(),
        picture: "".to_string(),
        email,
        email_verified: true,
    };

    let body = InsertOne::new(user);
    let _ = mutation!(body).token(Some(bearer.clone())).send(GRAPHQL_ENDPOINT).await;
}

async fn delete_hasura(token: String) {
    // TODO: also remove images from storage

    log::warn!("deleted the following quizzes: {info:?}");
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
        .unwrap()
        .json()
        .await
        .unwrap();

    let bearer = format!("Bearer {}", tokens.bearer);

    delete_hasura(bearer.clone()).await;
    delete_images(bearer.clone()).await;

    upload(bearer, tokens.id).await;
}
