use clap::Parser;
use rocket::{routes, Build, Rocket};
use rocket_cors::CorsOptions;
use sha3::Digest;
use sqlx::SqlitePool;

use routes::*;

mod error;
mod routes;

#[derive(clap::Parser)]
#[clap(version = "1.0", author = "Thomas Dooms <thomas@dooms.eu>")]
struct Opts {
    /// Sets the port to be used
    #[clap(short, long, default_value = "8004")]
    port: u16,

    /// Sets the ip address to be used
    #[clap(short, long, default_value = "127.0.0.1")]
    address: String,

    /// Sets the database to be used
    #[clap(short, long, default_value = "./backend/pixauth/db.sqlite")]
    database: String,
}

async fn setup(config: rocket::Config, path: String) -> Rocket<Build> {
    std::fs::OpenOptions::new().write(true).create(true).open(&path).unwrap();

    let url = format!("file:{path}");
    let pool = SqlitePool::connect(&url).await.unwrap();
    sqlx::query(include_str!("create.sql")).execute(&pool).await.unwrap();

    let email = std::env::var("ADMIN_EMAIL").unwrap();
    let password = std::env::var("ADMIN_PASSWORD").unwrap();

    let hash = &sha3::Sha3_256::new_with_prefix(password).finalize();
    let _ = sqlx::query("insert into users (email, pw_hash) values ($1, $2)")
        .bind(&email)
        .bind(base64::encode(&hash))
        .execute(&pool)
        .await;

    rocket::custom(config)
        .mount("/", routes![login, signup])
        .manage(pool)
        .attach(CorsOptions::default().to_cors().unwrap())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv()?;

    let opts: Opts = Opts::parse();
    let address = opts.address.parse()?;
    let config = rocket::Config { port: opts.port, address, ..Default::default() };

    let _ = setup(config, opts.database).await.launch().await?;

    Ok(())
}
