use rocket::{routes, Build, Rocket};
use rocket_cors::CorsOptions;
use sha3::Digest;
use sqlx::SqlitePool;

use routes::*;

mod error;
mod routes;

async fn setup(config: rocket::Config, path: &str) -> Rocket<Build> {
    std::fs::OpenOptions::new().write(true).create(true).open(path).unwrap();

    let url = format!("file:{path}");
    let pool = SqlitePool::connect(&url).await.unwrap();
    sqlx::query(include_str!("create.sql")).execute(&pool).await.unwrap();

    let email = std::env::var("ADMIN_EMAIL").unwrap();
    let password = std::env::var("ADMIN_PASSWORD").unwrap();

    sqlx::query("delete from users").execute(&pool).await.unwrap();

    let hash = &sha3::Sha3_256::new_with_prefix(password).finalize();
    let query = "insert into users (email, pw_hash) values ($1, $2) returning rowid, *";

    sqlx::query(query)
        .bind(&email)
        .bind(base64::encode(&hash))
        .fetch_one(&pool)
        .await
        .unwrap();

    rocket::custom(config)
        .mount("/", routes![login, signup])
        .manage(pool)
        .attach(CorsOptions::default().to_cors().unwrap())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv()?;

    let env = std::env::var("AUTH_ADDRESS")?;
    let vec: Vec<_> = env.split(':').collect();

    let address = vec[0].parse().map_err(|_| "invalid address")?;
    let port = vec[1].parse().map_err(|_| "invalid port")?;

    let database = std::env::var("AUTH_DATABASE")?;

    let config = rocket::Config { port, address, ..Default::default() };

    tracing::info!("listening on {}", address);
    let _ = setup(config, &database).await.launch().await?;

    Ok(())
}
