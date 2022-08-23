use crate::error::Error;
use crate::SqlitePool;
use jsonwebtoken::{encode, EncodingKey, Header};
use pixauth::{Claims, Credentials, HasuraClaims, Role, Tokens};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{delete, post, put, State};
use sha3::Digest;
use std::time::UNIX_EPOCH;

#[derive(sqlx::FromRow, Debug, Serialize)]
#[serde(crate = "rocket::serde")]
struct User {
    pub rowid: i64,
    pub role: i64,
    pub email: String,
    pub pw_hash: String,
}

fn create_jwt(user: &User) -> Result<String, Error> {
    let epoch = std::time::SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let expiration = 60 * 60;

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

    let secret = std::env::var("AUTH_SECRET")?;
    let encoding_key = EncodingKey::from_secret(secret.as_bytes());

    Ok(encode(&Header::default(), &claims, &encoding_key)?)
}

fn create_hash(password: &str) -> String {
    let hash = &sha3::Sha3_256::new_with_prefix(password).finalize();
    base64::encode(&hash)
}

fn generate_refresh() -> String {
    thread_rng().sample_iter(&Alphanumeric).take(32).map(char::from).collect()
}

#[post("/signup", data = "<body>")]
pub async fn signup(
    body: Json<Credentials>,
    pool: &State<SqlitePool>,
) -> Result<Json<Tokens>, Error> {
    let hash = create_hash(&body.password);

    let user: User =
        sqlx::query_as("insert into users (email, pw_hash) values ($1, $2) returning rowid, *")
            .bind(&body.email)
            .bind(&hash)
            .fetch_one(&**pool)
            .await?;

    println!("{user:?}");

    let refresh = generate_refresh();
    let id = format!("pixelguesser|{}", user.rowid);
    Ok(Json(Tokens { bearer: create_jwt(&user)?, refresh, id }))
}

#[post("/login", data = "<body>")]
pub async fn login(
    body: Json<Credentials>,
    pool: &State<SqlitePool>,
) -> Result<Json<Tokens>, Error> {
    let hash = create_hash(&body.password);

    let user: User = sqlx::query_as("select rowid, * from users where email=$1 and pw_hash=$2")
        .bind(&body.email)
        .bind(&hash)
        .fetch_one(&**pool)
        .await?;

    println!("{user:?}");

    let refresh = generate_refresh();
    let id = format!("pixelguesser|{}", user.rowid);
    Ok(Json(Tokens { bearer: create_jwt(&user)?, refresh, id }))
}

#[post("/refresh", data = "<body>")]
pub async fn refresh(body: &str, pool: &State<SqlitePool>) -> Result<Json<Tokens>, Error> {
    let refresh = generate_refresh();
    let user: User =
        sqlx::query_as("update users set refresh=$1 where refresh=$2 returning rowid, *")
            .bind(&refresh)
            .bind(body)
            .fetch_one(&**pool)
            .await?;

    println!("{user:?}");
    let id = format!("pixelguesser|{}", user.rowid);
    Ok(Json(Tokens { bearer: create_jwt(&user)?, refresh, id }))
}

// #[put("/update", data = "<body>")]
// pub async fn update(
//     body: Json<Credentials<'_>>,
//     token: Claims,
//     pool: &State<SqlitePool>,
// ) -> Result<Json<User>, Error> {
//     let user: User = sqlx::query_as("update users set email=$1, pw_hash=$2 where id=$3")
//         .bind(&body.email)
//         .bind(&body.password)
//         .bind(&token.sub)
//         .fetch_one(&**pool)
//         .await?;
//
//     Ok(Json(user))
// }
//
// #[delete("/")]
// pub async fn delete() -> Result<(), Error> {
//     Ok(())
// }
