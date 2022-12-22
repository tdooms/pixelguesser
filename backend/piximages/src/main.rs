use std::path::Path;

use base64::alphabet::URL_SAFE;
use base64::engine::fast_portable::{FastPortable, FastPortableConfig};
use blurhash_wasm::encode;
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
use piximages::{Resolution, Response};

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
pub async fn download(img: &str, resolution: &str, path: &State<Folder>) -> Option<NamedFile> {
    let path = format!("{}/{resolution}/{img}.jpg", path.0);
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
) -> Result<Json<Response>, Error> {
    let url = std::env::var("IMAGE_ENDPOINT").unwrap();
    let base64 = data.open(20.mebibytes()).into_string().await?;

    let buffer = base64::decode(&base64.value)?;
    let format = image::guess_format(&buffer)?;

    let engine = FastPortable::from(&URL_SAFE, FastPortableConfig::new());

    let hash = &sha3::Sha3_256::new_with_prefix(&buffer).finalize();
    let filename = base64::encode_engine(&hash, &engine);
    let base = path.inner().0.clone();

    sqlx::query("insert into owners (image, user) values (?1, ?2)")
        .bind(&filename)
        .bind(&token.sub)
        .execute(&**db)
        .await?;

    let original = image::load_from_memory_with_format(&buffer, format).unwrap();

    let (width, height) = original.dimensions();
    let vec = original.to_rgba8().into_raw();
    let blurhash = encode(vec, 4, 3, width as usize, height as usize).unwrap();

    let url = format!("{url}/{filename}");
    tokio::task::spawn_blocking(move || compute_image(&base, &filename, &original));

    Ok(Json(Response { url, blurhash }))
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
pub async fn reset(_token: Claims, path: &State<Folder>, db: &State<SqlitePool>) {
    // TODO: verify admin
    // token.sub = "admin".to_string();

    let base = &path.inner().0;

    sqlx::query("delete from owners").execute(&**db).await.unwrap();

    for res in [Resolution::Thumb, Resolution::Small, Resolution::HD, Resolution::Original] {
        std::fs::remove_dir_all(&format!("{}/{}", base, res)).unwrap();
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv()?;
    tracing_subscriber::fmt::init();

    let folder = std::env::var("IMAGE_FOLDER")?;
    let database = std::env::var("IMAGE_DATABASE")?;

    let env = std::env::var("IMAGE_ADDRESS")?;
    let vec: Vec<_> = env.split(':').collect();

    let address = vec[0].parse().map_err(|_| "invalid address")?;
    let port = vec[1].parse().map_err(|_| "invalid port")?;

    let config = rocket::Config { port, address, ..Default::default() };
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
        .manage(Folder(folder.to_owned()))
        .manage(db)
        .attach(cors)
        .launch()
        .await?;

    Ok(())
}
