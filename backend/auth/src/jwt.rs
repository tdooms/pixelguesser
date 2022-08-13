use std::time::UNIX_EPOCH;
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use crate::{Status};
use rocket::serde::{Deserialize, Serialize};
use crate::error::Error;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum Role {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "moderator")]
    Moderator,
}

#[derive(sqlx::FromRow, Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub rowid: i64,
    pub username: String,
    pub email: String,
    pub pw_hash: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DraftUser<'r> {
    pub email: &'r str,
    pub password: &'r str,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Login<'r> {
    pub email: &'r str,
    pub password: &'r str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct HasuraClaims {
    #[serde(rename = "x-hasura-default-role")]
    default_role: String,

    #[serde(rename = "x-hasura-allowed-roles")]
    allowed_roles: Vec<String>,

    #[serde(rename = "x-hasura-user-id")]
    user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Claims {
    pub sub: i64,
    pub exp: i64,

    #[serde(rename = "https://hasura.io/jwt/claims")]
    pub hasura: HasuraClaims,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Claims {
    type Error = Error;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token = match request.headers().get_one("Authorization"){
            Some(opt) => opt,
            None => return Outcome::Forward(()),
        };

        let mut iter = token.split(' ');

        let token = match (iter.next(), iter.next()) {
            (Some("Bearer"), Some(token)) => token,
            _ => return Outcome::Forward(()),
        };

        let secret = match std::env::var("SECRET_KEY") {
            Ok(secret) => secret,
            Err(_) => return Outcome::Failure((Status::InternalServerError, Error::NoSecretKey)),
        };

        let key = DecodingKey::from_secret(secret.as_bytes());

        let claims = match decode(token, &key, &Validation::default()){
            Ok(data) => data.claims,
            Err(_) => return Outcome::Forward(()),
        };

        Outcome::Success(claims)
    }
}

impl User {
    pub fn jwt(&self) -> Result<String, Error> {
        let epoch = std::time::SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let expiration = 60;

        let claims = Claims {
            sub: self.rowid,
            exp: epoch as i64 + expiration,
            hasura: HasuraClaims {
                default_role: "user".to_string(),
                allowed_roles: vec!["user".to_string()],
                user_id: format!("pixelguesser|{}", self.rowid),
            },
        };

        let secret = std::env::var("SECRET_KEY")?;
        let encoding_key = EncodingKey::from_secret(secret.as_bytes());

        Ok(encode(&Header::default(), &claims, &encoding_key)?)
    }
}