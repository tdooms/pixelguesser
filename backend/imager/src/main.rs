#[macro_use]
extern crate rocket;

use clap::Parser;
use photon_rs::{native::save_image, PhotonImage};
use rand::distributions::Alphanumeric;
use rand::Rng;
use rocket::data::ToByteUnit;
use rocket::fs::{FileServer, Options};
use rocket::{Config, Data, State};
use rocket_cors::{Cors, CorsOptions};

pub struct Path(String);

// #[post("/upload", format = "plain", data = "<bytes>")]
// pub async fn upload(bytes: Data<'_>, path: &State<Path>) -> std::io::Result<String> {
//     let filename = rand::thread_rng().sample_iter(&Alphanumeric).take(16).map(char::from).collect();
//
//     let filepath = format!("{}/{}.jpg", path.inner().0, filename);
//     let file = File::create(filepath).await?;
//     bytes.open(16.mebibytes()).stream_to(file).await?;
//
//     Ok(filename)
// }

#[post("/upload", data = "<data>")]
pub async fn upload(data: Data<'_>, path: &State<Path>) -> std::io::Result<String> {
    let random: String =
        rand::thread_rng().sample_iter(&Alphanumeric).take(16).map(char::from).collect();
    let filename = format!("{}.jpg", random);
    let filepath = format!("{}/{}", path.inner().0, filename);

    let base64 = data.open(10.mebibytes()).into_string().await?;

    save_image(PhotonImage::new_from_base64(&base64), &filepath);
    Ok(filename)
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
