use api::{login, Credentials, Quiz, User};
use std::fs::File;
use std::rc::Rc;

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

async fn inner() -> Result<(), Box<dyn std::error::Error>> {
    let email = std::env::var("ADMIN_EMAIL")?;
    let password = std::env::var("ADMIN_PASSWORD")?;
    let credentials = Rc::new(Credentials { email: email.clone(), password });

    let tokens = login(credentials).await?;

    let user = User {
        user_id: Some(tokens.id.parse()?),
        nickname: "admin".to_string(),
        // image: Image::default(),
        email,
        last_seen: None,
        verified: true,
    };

    let file = File::open("init/create.json")?;
    let Quizzes { mut quizzes } = serde_json::from_reader(file)?;

    let bearer = format!("Bearer {}", tokens.bearer);

    delete_quizzes(bearer.clone()).await?;
    delete_images(bearer.clone()).await?;
    delete_user(bearer.clone()).await?;

    upload_user(user, bearer.clone()).await?;
    upload_images(&mut quizzes, bearer.clone()).await?;
    upload_quizzes(&mut quizzes, tokens.id.parse()?, bearer).await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().unwrap();
    tracing_subscriber::fmt().init();

    if let Err(err) = inner().await {
        tracing::error!("{err}");
    }
}
