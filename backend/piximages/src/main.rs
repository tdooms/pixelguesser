use std::path::Path;

use base64::URL_SAFE;
use blurhash_wasm::encode;
use clap::Parser;
use image::{DynamicImage, GenericImageView};
use rocket::data::ToByteUnit;
use rocket::fs::NamedFile;
use rocket::http::Status;
use rocket::response::Responder;
use rocket::serde::json::Json;
use rocket::{get, post, response, routes, Data, Request, State};
use rocket_cors::CorsOptions;
use sha3::Digest;
use sqlx::{Row, SqlitePool};

use pixauth::Claims;
use piximages::{Resolution, UploadResult};

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

impl<'r, 'o: 'r> Responder<'r, 'o> for Error {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        println!("{:?}", self);
        Status::InternalServerError.respond_to(req)
    }
}

pub struct Folder(String);

#[get("/<img>/<resolution>")]
pub async fn download(img: &str, resolution: &str) -> Option<NamedFile> {
    let path = format!("backend/piximages/data/{resolution}/{img}.jpg");
    NamedFile::open(Path::new(&path)).await.ok()
}

fn compute_image(base: &str, filename: &str, original: &DynamicImage) {
    let _ = std::fs::create_dir(&format!("{base}/original"));
    original.save(&format!("{base}/original/{filename}.jpg")).unwrap();

    for res in [Resolution::Thumb, Resolution::Small, Resolution::HD] {
        let img = original.thumbnail(1_000_000, res as u32);
        let _ = std::fs::create_dir(&format!("{base}/{res}"));
        img.save(&format!("{base}/{res}/{filename}.jpg")).unwrap();
    }
}

#[post("/upload", data = "<data>")]
pub async fn upload(
    data: Data<'_>,
    token: Claims,
    path: &State<Folder>,
    db: &State<SqlitePool>,
) -> Result<Json<UploadResult>, Error> {
    let base64 = data.open(20.mebibytes()).into_string().await?;

    let buffer = base64::decode(&base64.value)?;
    let format = image::guess_format(&buffer)?;

    let hash = &sha3::Sha3_256::new_with_prefix(&buffer).finalize();
    let filename = base64::encode_config(&hash, URL_SAFE);
    let base = path.inner().0.clone();

    sqlx::query("insert into owners (image, user) values (?1, ?2)")
        .bind(&filename)
        .bind(&token.sub)
        .execute(&**db)
        .await?;

    let mut original = image::load_from_memory_with_format(&buffer, format).unwrap();

    let (width, height) = original.dimensions();
    let vec = original.to_rgba8().into_raw();
    let blurhash = encode(vec, 4, 3, width as usize, height as usize).unwrap();

    let url = format!("http://localhost:8901/{filename}");
    tokio::task::spawn_blocking(move || compute_image(&base, &filename, &original));

    Ok(Json(UploadResult { url, blurhash }))
}

#[post("/delete/<file>")]
pub async fn delete(
    file: &str,
    token: Claims,
    path: &State<Folder>,
    db: &State<SqlitePool>,
) -> Result<(), Error> {
    let base = &path.inner().0;
    let (filename, _) = file.rsplit_once('.').unwrap();

    sqlx::query("delete from owners where image = ?1 and user = ?2")
        .bind(&filename)
        .bind(&token.sub)
        .execute(&**db)
        .await?;

    let count = sqlx::query("select count(*) from owners where image = ?1")
        .bind(&filename)
        .fetch_one(&**db)
        .await?;

    if count.get::<u32, _>(0) != 0 {
        return Ok(());
    }

    for res in [Resolution::Thumb, Resolution::Small, Resolution::HD, Resolution::Original] {
        std::fs::remove_file(&format!("{}/{}/{}", base, res, file))?;
    }
    Ok(())
}

#[post("/reset")]
pub async fn reset(token: Claims, path: &State<Folder>, db: &State<SqlitePool>) {
    // TODO: verify admin
    // token.sub = "admin".to_string();

    let base = &path.inner().0;

    sqlx::query("delete from owners").execute(&**db).await.unwrap();

    for res in [Resolution::Thumb, Resolution::Small, Resolution::HD, Resolution::Original] {
        std::fs::remove_dir_all(&format!("{}/{}", base, res)).unwrap();
    }
}

/// imager (IMAGE-serveR) is a program to efficiently serve images
#[derive(Parser)]
#[clap(version = "1.0", author = "Thomas Dooms <thomas@dooms.eu>")]
struct Opts {
    /// Sets the folder to be served
    #[clap(short, long, default_value = "./backend/piximages/data")]
    folder: String,

    /// Sets the database to be used
    #[clap(short, long, default_value = "./backend/piximages/db.sqlite")]
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
    dotenv::dotenv().unwrap();
    let Opts { database, port, address, folder } = Opts::parse();

    let config = rocket::Config { port, address: address.parse()?, ..Default::default() };
    let cors = CorsOptions::default().to_cors()?;

    let _ = std::fs::create_dir(&folder);

    let res = [Resolution::Thumb, Resolution::Small, Resolution::HD, Resolution::Original];
    for resolution in res {
        let _ = std::fs::create_dir(&format!("{folder}/{resolution}"));
    }

    std::fs::OpenOptions::new().write(true).create(true).open(&database)?;

    let url = format!("file:{database}");
    let db = SqlitePool::connect(&url).await?;

    sqlx::query(include_str!("create.sql")).execute(&db).await?;

    let _ = rocket::custom(config)
        .mount("/", routes![upload, download, reset])
        .manage(Folder(folder))
        .manage(db)
        .attach(cors)
        .launch()
        .await?;

    Ok(())
}
