use std::net::IpAddr;
use rocket::{routes};
use rocket::figment::Figment;
use rocket::figment::providers::{Format, Toml};
use rocket_cors::CorsOptions;
use serde::Deserialize;
use sha3::Digest;
use sqlx::{Pool, Sqlite, SqlitePool};

use routes::*;

mod error;
mod routes;

#[derive(Deserialize)]
struct Config {
    address: IpAddr,
    port: u16,
    database: String,
    secret: String,
    email: String,
    password: String,
}

async fn setup(database: &str, email: &str, password: &str) -> Pool<Sqlite> {
    std::fs::OpenOptions::new().write(true).create(true).open(database).unwrap();

    let url = format!("file:{database}");
    let pool = SqlitePool::connect(&url).await.unwrap();
    sqlx::query(include_str!("create.sql")).execute(&pool).await.unwrap();

    sqlx::query("delete from users").execute(&pool).await.unwrap();

    let hash = &sha3::Sha3_256::new_with_prefix(password).finalize();
    let query = "insert into users (email, pw_hash) values ($1, $2) returning rowid, *";

    sqlx::query(query)
        .bind(email)
        .bind(base64::encode(&hash))
        .fetch_one(&pool)
        .await
        .unwrap();

    pool
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let provider = Toml::file("config.toml").nested();
    let config: Config = Figment::from(provider).select("auth").extract().unwrap();
    let Config{ address, port, secret, email, password, database } = config;

    let pool = setup(&database, &email, &password).await;
    tracing::info!("listening on {address}:{port}");

    let config = rocket::Config { port, address, ..Default::default() };
    let _ = rocket::custom(config)
        .mount("/", routes![login, signup])
        .manage(pool)
        .manage(secret)
        .attach(CorsOptions::default().to_cors().unwrap())
        .launch()
        .await?;

    Ok(())
}
