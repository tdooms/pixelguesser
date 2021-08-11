#![feature(generators, generator_trait)]

#[macro_use]
extern crate rocket;

use clap::{AppSettings, Clap};
use rocket::Config;
use crate::routes::*;
use crate::state::Sessions;

mod routes;
mod state;
mod util;

/// sessions is a server to manage pixelguesser game sessions
#[derive(Clap)]
#[clap(version = "1.0", author = "Thomas Dooms <thomas@dooms.eu>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// Sets the port to be used
    #[clap(short, long, default_value = "8000")]
    port: u16,

    /// Sets the ip address to be used
    #[clap(short, long, default_value = "127.0.0.1")]
    address: String,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts: Opts = Opts::parse();

    let config = Config {
        port: opts.port,
        address: opts.address.parse()?,
        ..Default::default()
    };

    rocket::custom(config)
        .mount("/", routes![create, read, update, check, subscribe])
        .manage(Sessions::default())
        .launch()
        .await?;

    Ok(())
}
