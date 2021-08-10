#[macro_use]
extern crate rocket;

use clap::{AppSettings, Clap};
use rocket::{Config, Request};
use rocket::fs::{FileServer, NamedFile, Options};

/// spar (SPA-serveR) is a simple program to serve a folder for serving single page applications.
#[derive(Clap)]
#[clap(version = "1.0", author = "Thomas Dooms <thomas@dooms.eu>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// Sets the folder to be served
    #[clap(short, long, default_value = "./static")]
    folder: String,

    /// Sets the port to be used
    #[clap(short, long, default_value = "8000")]
    port: u16,

    /// Sets the ip address to be used
    #[clap(short, long, default_value = "127.0.0.1")]
    address: String,
}

pub struct Path(String);


#[catch(404)]
async fn not_found(request: &Request<'_>) -> Result<NamedFile, String> {
    // let index = request.rocket().figment().find_value("folder").unwrap();
    // let path = format!("{}/index.html", index.as_str().unwrap());

    let folder = request.rocket().state().get_or_insert(&Path("./static".to_owned())).0.clone();
    let index = format!("{}/index.html", folder);

    NamedFile::open(index).await.map_err(|_| "could not find path".to_owned())
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
        .mount("/", FileServer::new(&opts.folder, Options::default()))
        .register("/", catchers![not_found])
        .manage(Path(opts.folder))
        .launch()
        .await?;

    Ok(())
}