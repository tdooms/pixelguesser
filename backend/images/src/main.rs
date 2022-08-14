use base64::URL_SAFE;
use clap::Parser;
use image::ImageFormat;
use images::Resolution;
use rocket::data::ToByteUnit;
use rocket::fs::NamedFile;
use rocket::http::Status;
use rocket::response::Responder;
use rocket::{response, Data, Request, State};
use rocket_cors::CorsOptions;
use sha3::Digest;
use sqlx::{Row, SqlitePool};
use std::path::Path;

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

impl<'r, 'o: 'r> Responder<'r, 'o> for Error {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        println!("{:?}", self);
        Status::InternalServerError.respond_to(req)
    }
}

fn precompute_image(buffer: Vec<u8>, format: ImageFormat) {
    let original = image::load_from_memory_with_format(&buffer, format)?;
    original.save(&format!("{base}/original/{filename}.jpg")).unwrap();

    for res in [Resolution::Thumb, Resolution::Small, Resolution::HD] {
        let img = original.thumbnail(1_000_000, res as u32);
        img.save(&format!("{base}/{res}/{filename}.jpg")).unwrap();
    }
}

#[get("/<img>/<kind>")]
pub async fn download(img: &str, kind: &str) -> Option<NamedFile> {
    let path = format!("backend/images/data/{kind}/{img}.jpg");
    NamedFile::open(Path::new(&path)).await.ok()
}

#[post("/upload", data = "<data>")]
pub async fn upload(
    data: Data<'_>,
    token: &auth::Claims,
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
    tokio::task::spawn_blocking(precompute_image(buffer, format));

    sqlx::query("insert into owners (image, user) values (?1, ?2)")
        .bind(&filename)
        .bind(token.sub)
        .execute(&**db)
        .await?;

    let endpoint = "http://localhost:8901";
    Ok(format!("{endpoint}/{filename}"))
}

#[post("/delete/<file>")]
pub async fn delete(
    file: &str,
    token: &auth::Claims,
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

    for res in [Resolution::Thumb, Resolution::Small, Resolution::HD, Resolution::Original] {
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

    let _ = std::fs::create_dir(&opts.folder);

    let res = [Resolution::Thumb, Resolution::Small, Resolution::HD, Resolution::Original];
    for resolution in res {
        let _ = std::fs::create_dir(&format!("{}/{}", opts.folder, resolution));
    }
    let _vec: Vec<_> = res.into_iter().map(|x| format!("{}/{x}", opts.folder)).collect();

    if !std::path::Path::new(&opts.database).exists() {
        std::fs::File::create(&opts.database)?;
    }

    let url = format!("file:{}", opts.database);
    let db = SqlitePool::connect(&url).await?;

    sqlx::query("create table if not exists owners (image text, user text)").execute(&db).await?;

    let _ = rocket::custom(config)
        .mount("/", routes![upload, download])
        .manage(Folder(opts.folder))
        .manage(db)
        .attach(cors)
        .launch()
        .await?;

    Ok(())
}
