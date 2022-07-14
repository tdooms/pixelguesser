#[macro_use]
extern crate rocket;

use clap::Parser;
use image::imageops::FilterType;
use rand::distributions::Alphanumeric;
use rand::Rng;
use rocket::data::ToByteUnit;
use rocket::fs::{FileServer, Options};
use rocket::http::Status;
use rocket::response::Responder;
use rocket::{response, Config, Data, Request, State};
use rocket_cors::CorsOptions;

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
        println!("{:?}", self);
        Status::InternalServerError.respond_to(req)
    }
}

#[post("/upload", data = "<data>")]
pub async fn upload(data: Data<'_>, path: &State<Path>) -> Result<String, Error> {
    let base64 = data.open(10.mebibytes()).into_string().await?;

    let buffer = base64::decode(&base64.value)?;
    let format = image::guess_format(&buffer)?;

    let rng = rand::thread_rng();
    let random: String = rng.sample_iter(&Alphanumeric).take(16).map(char::from).collect();
    let extension = format.extensions_str().first().unwrap();
    let base = &path.inner().0;

    let original = image::load_from_memory_with_format(&buffer, format)?;
    original.save(&format!("{base}/original/{random}.{extension}"))?;

    let thumbnail = original.resize(1_000_000, 108, FilterType::Lanczos3);
    thumbnail.save(&format!("{base}/thumbnail/{random}.{extension}"))?;

    let card = original.resize(1_000_000, 320, FilterType::Lanczos3);
    card.save(&format!("{base}/card/{random}.{extension}"))?;

    let hd = original.resize(1_000_000, 1080, FilterType::Lanczos3);
    hd.save(&format!("{base}/hd/{random}.{extension}"))?;

    println!("{} {}", original.width(), original.height());
    Ok(format!("{}.{}", random, extension))
}

pub fn upload_template(templates: impl AsRef<std::path::Path>, base: impl std::fmt::Display) {
    // std::fs::create_dir(&format!("{base}/original")).unwrap();
    // std::fs::create_dir(&format!("{base}/thumbnail")).unwrap();
    // std::fs::create_dir(&format!("{base}/card")).unwrap();
    // std::fs::create_dir(&format!("{base}/hd")).unwrap();

    println!("start computing caches");

    for file in std::fs::read_dir(templates).unwrap() {
        let name = file.as_ref().unwrap().file_name().into_string().unwrap();

        println!("computing caches for image: {name}");

        let original = image::open(&file.unwrap().path()).unwrap();

        if !std::path::Path::new(&format!("{base}/original/{name}")).exists() {
            original.save(&format!("{base}/original/{name}")).unwrap();
        }
        if !std::path::Path::new(&format!("{base}/thumbnail/{name}")).exists() {
            let thumbnail = original.resize(1_000_000, 108, FilterType::Lanczos3);
            thumbnail.save(&format!("{base}/thumbnail/{name}")).unwrap();
        }

        if !std::path::Path::new(&format!("{base}/card/{name}")).exists() {
            let card = original.resize(1_000_000, 320, FilterType::Lanczos3);
            card.save(&format!("{base}/card/{name}")).unwrap();
        }
        if !std::path::Path::new(&format!("{base}/hd/{name}")).exists() {
            let hd = original.resize(1_000_000, 1080, FilterType::Lanczos3);
            hd.save(&format!("{base}/hd/{name}")).unwrap();
        }
    }

    println!("end computing caches");
}

/// imager (IMAGE-serveR) is a program to efficiently serve images
#[derive(Parser)]
#[clap(version = "1.0", author = "Thomas Dooms <thomas@dooms.eu>")]
struct Opts {
    /// Sets the folder to be served
    #[clap(short, long, default_value = "./data")]
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
    // upload_template("./data/template", "./data");

    let config = Config { port: opts.port, address: opts.address.parse()?, ..Default::default() };

    let cors = CorsOptions::default().to_cors()?;

    rocket::custom(config)
        .mount("/original", FileServer::new("data/original", Options::Index))
        .mount("/hd", FileServer::new("data/hd", Options::Index))
        .mount("/card", FileServer::new("data/card", Options::Index))
        .mount("/thumbnail", FileServer::new("data/thumbnail", Options::Index))
        .mount("/", routes![upload])
        .manage(Path(opts.folder))
        .attach(cors)
        .launch()
        .await?;

    Ok(())
}
