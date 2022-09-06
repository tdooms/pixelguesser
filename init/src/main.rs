use api::{Credentials, DraftQuiz, Image, Quiz, Tokens, User, AUTH_ENDPOINT, GRAPHQL_ENDPOINT};
use hasura::{
    mutation, Delete, DeleteBuilder, Insert, InsertBuilder, InsertOne, InsertOneBuilder, Object,
};
use std::fs::File;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Quizzes {
    quizzes: Vec<Quiz>,
}

async fn convert_image(image: &mut Image, token: String) {
    let filename = (*std::mem::take(image).url().unwrap()).clone();
    let path = format!("init/images/{filename}");
    log::info!("uploading image: {path}");

    let bytes = std::fs::read(&path).unwrap();
    let base64 = base64::encode(&bytes);

    *image = Image::from_base64(base64, Some(path));
    image.upload(token).await.unwrap();
    log::error!("{:?}", image.url());
}

async fn upload(token: String, creator_id: String) {
    let file = File::open("init/create.json").unwrap();
    let Quizzes { mut quizzes } = serde_json::from_reader(file).unwrap();

    for quiz in &mut quizzes {
        quiz.creator_id = Some(creator_id.clone());
        convert_image(&mut quiz.image, token.clone()).await;

        for (index, round) in &mut quiz.rounds.iter_mut().enumerate() {
            convert_image(&mut round.image, token.clone()).await;
            round.index = index as u64
        }
    }

    let insert = Insert::new(quizzes);
    let inserted = mutation!(insert).token(Some(token)).send(GRAPHQL_ENDPOINT).await.unwrap();

    let info: Vec<_> = inserted.into_iter().map(|x| x.title).collect();
    log::info!("uploaded the following quizzes: {:?}", info);
}

async fn delete(token: String) {
    // TODO: also remove images from storage

    let delete = Delete::new();
    let deleted = mutation!(delete).token(Some(token)).send(GRAPHQL_ENDPOINT).await.unwrap();

    let info: Vec<_> = deleted.into_iter().map(|x| x.title).collect();
    log::warn!("deleted the following quizzes: {info:?}");
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenv::dotenv().unwrap();

    let email = std::env::var("ADMIN_EMAIL").unwrap();
    let password = std::env::var("ADMIN_PASSWORD").unwrap();
    let credentials = Credentials { email: email.clone(), password };

    let tokens: Tokens = reqwest::Client::new()
        .post(format!("{AUTH_ENDPOINT}/login"))
        .json(&credentials)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let bearer = format!("Bearer {}", tokens.bearer);

    let user = User {
        id: tokens.id.clone(),
        nickname: "thomas".to_string(),
        picture: "".to_string(),
        email,
        email_verified: true,
    };

    let body = InsertOne::new(user);
    let _ = mutation!(body).token(Some(bearer.clone())).send(GRAPHQL_ENDPOINT).await;

    delete(bearer.clone()).await;
    upload(bearer, tokens.id).await;
}
