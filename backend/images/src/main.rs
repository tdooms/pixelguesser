#[macro_use]
extern crate rocket;

use base64::URL_SAFE;
use clap::Parser;
use images::Resolution;
use rocket::data::ToByteUnit;
use rocket::fs::{FileServer, Options};
use rocket::http::Status;
use rocket::response::Responder;
use rocket::{response, Data, Request, State};
use rocket_cors::CorsOptions;
use sha3::Digest;
use sqlx::{Row, SqlitePool};

pub struct Folder(String);

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("error reading body into string: {0:?}")]
    Body(#[from] std::io::Error),

    #[error("error decoding base 64: {0:?}")]
    Base64(#[from] base64::DecodeError),

    #[error("error manipulating image: {0:?}")]
    Image(#[from] image::ImageError),

    #[error("error writing to database: {0:?}")]
    Sqlx(#[from] sqlx::Error),
}

// This is ugly
impl<'r, 'o: 'r> Responder<'r, 'o> for Error {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        println!("{:?}", self);
        Status::InternalServerError.respond_to(req)
    }
}

#[post("/upload/<user>", data = "<data>")]
pub async fn upload(
    data: Data<'_>,
    user: &str,
    path: &State<Folder>,
    db: &State<SqlitePool>,
) -> Result<String, Error> {
    let base64 = data.open(20.mebibytes()).into_string().await?;

    let buffer = base64::decode(&base64.value)?;
    let format = image::guess_format(&buffer)?;

    let hash = &sha3::Sha3_256::new_with_prefix(&buffer).finalize();
    let filename = base64::encode_config(&hash, URL_SAFE);
    let extension = format.extensions_str().first().unwrap();

    let base = &path.inner().0;

    println!("{base}/original/{filename}.{extension}");
    let original = image::load_from_memory_with_format(&buffer, format)?;
    original.save(&format!("{base}/original/{filename}.{extension}"))?;

    for res in [Resolution::Thumbnail, Resolution::Card, Resolution::HD] {
        let img = original.thumbnail(1_000_000, res as u32);
        img.save(&format!("{base}/{res}/{filename}.{extension}"))?;
    }

    sqlx::query("insert into owners (image, user) values (?1, ?2)")
        .bind(&filename)
        .bind(&user)
        .execute(&**db)
        .await?;

    Ok(format!("{}.{}", filename, extension))
}

#[post("/delete/<file>/<user>")]
pub async fn delete(
    file: &str,
    user: &str,
    path: &State<Folder>,
    db: &State<SqlitePool>,
) -> Result<(), Error> {
    let base = &path.inner().0;
    let (filename, _) = file.rsplit_once('.').unwrap();

    sqlx::query("delete from owners where image = ?1 and user = ?2")
        .bind(&filename)
        .bind(&user)
        .execute(&**db)
        .await?;

    let count = sqlx::query("select count(*) from owners where image = ?1")
        .bind(&filename)
        .fetch_one(&**db)
        .await?;

    if count.get::<u32, _>(0) != 0 {
        return Ok(());
    }

    for res in [Resolution::Thumbnail, Resolution::Card, Resolution::HD, Resolution::Original] {
        std::fs::remove_file(&format!("{}/{}/{}", base, res, file))?;
    }
    Ok(())
}

/// imager (IMAGE-serveR) is a program to efficiently serve images
#[derive(Parser)]
#[clap(version = "1.0", author = "Thomas Dooms <thomas@dooms.eu>")]
struct Opts {
    /// Sets the folder to be served
    #[clap(short, long, default_value = "./backend/images/data")]
    folder: String,

    /// Sets the folder to be served
    #[clap(short, long, default_value = "./backend/images/data/db.sqlite")]
    database: String,

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

    let address = opts.address.parse()?;
    let config = rocket::Config { port: opts.port, address, ..Default::default() };

    let cors = CorsOptions::default().to_cors()?;

    let res = [Resolution::Thumbnail, Resolution::Card, Resolution::HD, Resolution::Original];
    for resolution in res {
        let _ = std::fs::create_dir(&format!("{}/{}", opts.folder, resolution));
    }
    let vec: Vec<_> = res.into_iter().map(|x| format!("{}/{x}", opts.folder)).collect();

    if !std::path::Path::new(&opts.database).exists() {
        std::fs::File::create(&opts.database)?;
    }

    let url = format!("file:{}", opts.database);
    let db = SqlitePool::connect(&url).await?;

    sqlx::query("create table if not exists owners (image text, user text)").execute(&db).await?;

    let _ = rocket::custom(config)
        .mount("/original", FileServer::new(&vec[3], Options::Index))
        .mount("/hd", FileServer::new(&vec[2], Options::Index))
        .mount("/card", FileServer::new(&vec[1], Options::Index))
        .mount("/thumbnail", FileServer::new(&vec[0], Options::Index))
        .mount("/", routes![upload])
        .manage(Folder(opts.folder))
        .manage(db)
        .attach(cors)
        .launch()
        .await?;

    Ok(())
}
