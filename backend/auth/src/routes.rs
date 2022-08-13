use rocket::serde::json::Json;
use sha3::Digest;
use crate::{Claims, SqlitePool, User};
use crate::error::Error;
use rocket::{post, put, delete, State};
use crate::jwt::{DraftUser, Login};

fn generate_hash(password: &str) -> String {
    let hash = &sha3::Sha3_256::new_with_prefix(password).finalize();
    base64::encode(&hash)
}

#[post("/signup", data = "<body>")]
pub async fn signup(body: Json<DraftUser<'_>>, pool: &State<SqlitePool>) -> Result<(), Error> {
    let hash = generate_hash(body.password);

    sqlx::query("insert into users (email, pw_hash) values ($1, $2)")
        .bind(&body.email)
        .bind(&hash)
        .execute(&**pool).await?;

    Ok(())
}

#[post("/login", data = "<body>")]
pub async fn login(body: Json<Login<'_>>, pool: &State<SqlitePool>) -> Result<String, Error> {
    let hash = generate_hash(body.password);

    let user: User = sqlx::query_as("select rowid, * from users where email=$1 and pw_hash=$2")
        .bind(&body.email)
        .bind(&hash)
        .fetch_one(&**pool).await?;

    println!("{user:?}");

    user.jwt()
}

#[put("/update", data = "<body>")]
pub async fn update(body: Json<DraftUser<'_>>, token: Claims, pool: &State<SqlitePool>) -> Result<Json<User>, Error> {
    let user: User = sqlx::query_as("update users set email=$1, pw_hash=$2 where id=$3")
        .bind(&body.email)
        .bind(&body.password)
        .bind(&token.sub)
        .fetch_one(&**pool).await?;

    Ok(Json(user))
}

#[put("/", data = "<_body>")]
pub async fn update_fail(_body: Json<DraftUser<'_>>) -> Error {
    Error::AuthorizationFailed
}

#[delete("/")]
pub async fn delete() -> Result<(), Error> {
    Ok(())
}