mod jwt;
mod test;
mod routes;
mod error;

use rocket::http::Status;

use sqlx::SqlitePool;
use rocket_cors::CorsOptions;
use clap::Parser;
use rocket::{Build, Rocket, routes};
use crate::jwt::{Claims, User};
use routes::*;

#[derive(clap::Parser)]
#[clap(version = "1.0", author = "Thomas Dooms <thomas@dooms.eu>")]
struct Opts {
    /// Sets the port to be used
    #[clap(short, long, default_value = "8004")]
    port: u16,

    /// Sets the ip address to be used
    #[clap(short, long, default_value = "127.0.0.1")]
    address: String,
}

async fn setup(config: rocket::Config) -> Result<Rocket<Build>, Box<dyn std::error::Error>> {
    let db = "db.sqlite";
    std::fs::OpenOptions::new().write(true).create(true).open(db)?;

    let url = format!("file:{db}");
    let db = SqlitePool::connect(&url).await?;
    sqlx::query(include_str!("create.sql")).execute(&db).await?;

    let rocket = rocket::custom(config)
        .mount("/", routes![login, signup, update, update_fail, delete])
        .manage(db)
        .attach(CorsOptions::default().to_cors()?);

    Ok(rocket)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv()?;

    let opts: Opts = Opts::parse();
    let address = opts.address.parse()?;
    let config = rocket::Config { port: opts.port, address, ..Default::default() };

    let _ = setup(config).await?.launch().await?;

    Ok(())
}
