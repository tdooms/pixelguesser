use api::{Credentials, DraftQuiz, Image, Quiz, Tokens, User, AUTH_ENDPOINT, GRAPHQL_ENDPOINT};
use hasura::{mutation, DeleteBuilder, InsertBuilder, InsertOneBuilder, Object};
use std::fs::File;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Quizzes {
    quizzes: Vec<DraftQuiz>,
}

async fn convert_image(image: &mut Image, token: String) {
    let filename = (*std::mem::take(image).url().unwrap()).clone();
    let path = format!("init/images/{filename}");
    log::info!("uploading image: {path}");

    let bytes = std::fs::read(&path).unwrap();
    let base64 = base64::encode(&bytes);

    *image = Image::from_base64(base64, Some(path));
    image.upload(token).await.unwrap();
}

async fn upload(token: String, creator: String) {
    let file = File::open("init/create.json").unwrap();
    let Quizzes { mut quizzes } = serde_json::from_reader(file).unwrap();

    for draft in &mut quizzes {
        draft.creator_id = Some(creator.clone());
        convert_image(&mut draft.image, token.clone()).await;
        log::error!("{:?}", draft.image.url());

        for (index, round) in &mut draft.rounds.data.iter_mut().enumerate() {
            convert_image(&mut round.image, token.clone()).await;
            round.index = index as u32
        }
    }

    log::info!("uploading quizzes");

    let insert = InsertBuilder::default().returning(Quiz::all()).objects(quizzes).build().unwrap();
    log::warn!("{}", insert);
    let inserted = mutation!(insert).token(Some(token)).send(GRAPHQL_ENDPOINT).await.unwrap();

    let quiz_ids: Vec<_> = inserted.into_iter().map(|x| x.id).collect();
    log::info!("uploaded the following quizzes: {:?}", quiz_ids);
}

async fn delete(token: String) {
    // TODO: also remove images from storage
    let delete = DeleteBuilder::default().returning(Quiz::all()).build().unwrap();
    let quizzes = mutation!(delete).token(Some(token)).send(GRAPHQL_ENDPOINT).await.unwrap();

    let info: Vec<_> = quizzes.into_iter().map(|x| x.title).collect();
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

    let body = InsertOneBuilder::default().object(user).returning(User::all()).build().unwrap();
    let _ = mutation!(body).token(Some(bearer.clone())).send(GRAPHQL_ENDPOINT).await;

    delete(bearer.clone()).await;
    upload(bearer, tokens.id).await;
}
