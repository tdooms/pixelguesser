use crate::error::Error;
use crate::SqlitePool;
use auth::{Claims, HasuraClaims, Role};
use jsonwebtoken::{encode, EncodingKey, Header};
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{delete, post, put, State};
use sha3::Digest;
use std::time::UNIX_EPOCH;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Credentials<'r> {
    pub email: &'r str,
    pub password: &'r str,
}

#[derive(sqlx::FromRow, Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub rowid: i64,
    pub role: i64,
    pub email: String,
    pub pw_hash: String,
}

fn create_jwt(user: &User) -> Result<String, Error> {
    let epoch = std::time::SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let expiration = 60;

    let role = match user.role {
        0 => Role::User,
        1 => Role::Moderator,
        2 => Role::Admin,
        _ => return Err(Error::InvalidRole),
    };

    let user_id = format!("pixelguesser|{}", user.rowid);
    let default_role = Role::User;

    let hasura = HasuraClaims { default_role, allowed_roles: vec![role], user_id: user_id.clone() };
    let claims = Claims { sub: user_id, exp: epoch as i64 + expiration, role, hasura };

    let secret = std::env::var("SECRET_KEY")?;
    let encoding_key = EncodingKey::from_secret(secret.as_bytes());

    Ok(encode(&Header::default(), &claims, &encoding_key)?)
}

fn create_hash(password: &str) -> String {
    let hash = &sha3::Sha3_256::new_with_prefix(password).finalize();
    base64::encode(&hash)
}

#[post("/signup", data = "<body>")]
pub async fn signup(body: Json<Credentials<'_>>, pool: &State<SqlitePool>) -> Result<(), Error> {
    let hash = create_hash(body.password);

    sqlx::query("insert into users (email, pw_hash) values ($1, $2)")
        .bind(&body.email)
        .bind(&hash)
        .execute(&**pool)
        .await?;

    Ok(())
}

#[post("/login", data = "<body>")]
pub async fn login(body: Json<Credentials<'_>>, pool: &State<SqlitePool>) -> Result<String, Error> {
    let hash = create_hash(body.password);

    let user: User = sqlx::query_as("select rowid, * from users where email=$1 and pw_hash=$2")
        .bind(&body.email)
        .bind(&hash)
        .fetch_one(&**pool)
        .await?;

    println!("{user:?}");

    create_jwt(&user)
}

#[put("/update", data = "<body>")]
pub async fn update(
    body: Json<Credentials<'_>>,
    token: Claims,
    pool: &State<SqlitePool>,
) -> Result<Json<User>, Error> {
    let user: User = sqlx::query_as("update users set email=$1, pw_hash=$2 where id=$3")
        .bind(&body.email)
        .bind(&body.password)
        .bind(&token.sub)
        .fetch_one(&**pool)
        .await?;

    Ok(Json(user))
}

#[delete("/")]
pub async fn delete() -> Result<(), Error> {
    Ok(())
}
