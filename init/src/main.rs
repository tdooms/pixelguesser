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

#[derive(Clone)]
struct Config {
    email: String,
    password: String,
    quizzes: String,
}

async fn inner(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let Config{ email, password, .. } = config.clone();
    let credentials = Rc::new(Credentials { email, password });

    let tokens = login(credentials).await?;

    let user = User {
        user_id: Some(tokens.id.parse()?),
        nickname: "thomas".to_string(),
        // image: Image::default(),
        email: config.email.clone(),
        last_seen: None,
        verified: true,
    };

    let file = File::open(&config.quizzes)?;
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
    tracing_subscriber::fmt::init();

    let provider = Toml::file("config.toml").nested();
    let config: Config = Figment::from(provider).select("auth").extract().unwrap();

    if let Err(err) = inner(&config).await {
        tracing::error!("{err}");
    }
}
