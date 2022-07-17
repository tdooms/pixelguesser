#[macro_use]
extern crate rocket;

use base64::{CharacterSet, Config};
use clap::Parser;
use image::imageops::FilterType;
use images::Resolution;
use rocket::data::ToByteUnit;
use rocket::fs::{FileServer, Options};
use rocket::http::Status;
use rocket::response::Responder;
use rocket::{response, Data, Request, State};
use rocket_cors::CorsOptions;
use sha3::Digest;

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

    let hash = &sha3::Sha3_256::new_with_prefix(&buffer).finalize();
    let filename = base64::encode(&hash);
    let extension = format.extensions_str().first().unwrap();

    let base = &path.inner().0;

    let original = image::load_from_memory_with_format(&buffer, format)?;
    original.save(&format!("{base}/original/{filename}.{extension}"))?;

    for res in [Resolution::Thumbnail, Resolution::Card, Resolution::HD] {
        let img = original.thumbnail(1_000_000, res as u32);
        img.save(&format!("{base}/{res}/{filename}.{extension}"))?;
    }

    Ok(format!("{}.{}", filename, extension))
}

pub fn upload_template(templates: impl AsRef<std::path::Path>, base: impl std::fmt::Display) {
    println!("computing image caches from template");

    for res in [Resolution::Thumbnail, Resolution::Card, Resolution::HD, Resolution::Original] {
        let _ = std::fs::create_dir(&format!("{}/{}", base, res));
    }

    for file in std::fs::read_dir(templates).unwrap() {
        let path = file.as_ref().unwrap().path();

        println!("\t {}", path.display());
        let original = image::open(&path).unwrap();

        let hash = sha3::Sha3_256::new_with_prefix(original.as_bytes()).finalize();
        let filename = base64::encode_config(&hash, Config::new(CharacterSet::UrlSafe, true));

        let _ = std::fs::copy(path, &format!("{base}/original/{filename}.jpg"));

        for res in [Resolution::Thumbnail, Resolution::Card, Resolution::HD] {
            let path = format!("{base}/{res}/{filename}.jpg");
            if !std::path::Path::new(&path).exists() {
                let image = original.resize(1_000_000, res as u32, FilterType::Lanczos3);
                image.save(&path).unwrap();
            }
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

    let config =
        rocket::Config { port: opts.port, address: opts.address.parse()?, ..Default::default() };

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
