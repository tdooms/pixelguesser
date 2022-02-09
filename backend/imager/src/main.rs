#[macro_use]
extern crate rocket;

use clap::Parser;
use rand::distributions::Alphanumeric;
use rand::Rng;
use rocket::data::ToByteUnit;
use rocket::fs::{FileServer, Options};
use rocket::http::Status;
use rocket::response::{Debug, Responder};
use rocket::{response, Config, Data, Request, State};
use rocket_cors::{Cors, CorsOptions};

pub struct Path(String);

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("error reading body into string: {0:?}")]
    Body(#[from] std::io::Error),

    #[error("error decoding base 64: {0:?}")]
    Base64(#[from] base64::DecodeError),

    #[error("error manipulating image: {0:?}")]
    Image(#[from] image::ImageError),
}

// This is ugly
impl<'r, 'o: 'r> Responder<'r, 'o> for Error {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        Status::InternalServerError.respond_to(req)
    }
}

#[post("/upload", data = "<data>")]
pub async fn upload(data: Data<'_>, path: &State<Path>) -> Result<String, Error> {
    let random: String =
        rand::thread_rng().sample_iter(&Alphanumeric).take(16).map(char::from).collect();

    let base64 = data.open(10.mebibytes()).into_string().await?;

    let buffer = base64::decode(&base64.value)?;
    let format = image::guess_format(&buffer)?;
    let image = image::load_from_memory_with_format(&buffer, format)?;

    let extension = format.extensions_str().first().unwrap();

    let filepath = format!("{}/{}.{}", path.inner().0, random, extension);
    image.save(&filepath)?;
    Ok(filepath)
}

/// imager (IMAGE-serveR) is a program to efficiently serve images
#[derive(Parser)]
#[clap(version = "1.0", author = "Thomas Dooms <thomas@dooms.eu>")]
struct Opts {
    /// Sets the folder to be served
    #[clap(short, long, default_value = "./images")]
    folder: String,

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

    let config = Config { port: opts.port, address: opts.address.parse()?, ..Default::default() };

    let cors = CorsOptions::default().to_cors()?;

    rocket::custom(config)
        .mount("/", FileServer::new(&opts.folder, Options::default()))
        .mount("/", routes![upload])
        .manage(Path(opts.folder))
        .attach(cors)
        .launch()
        .await?;

    Ok(())
}
