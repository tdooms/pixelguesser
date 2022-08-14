use jsonwebtoken::{decode, DecodingKey, Validation};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::serde::{Deserialize, Serialize};
use rocket::Request;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum Role {
    #[serde(rename = "user")]
    User = 0,
    #[serde(rename = "moderator")]
    Moderator = 1,
    #[serde(rename = "admin")]
    Admin = 2,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct HasuraClaims {
    #[serde(rename = "x-hasura-default-role")]
    pub default_role: Role,

    #[serde(rename = "x-hasura-allowed-roles")]
    pub allowed_roles: Vec<Role>,

    #[serde(rename = "x-hasura-user-id")]
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Claims {
    pub sub: String,
    pub exp: i64,

    pub role: Role,

    #[serde(rename = "https://hasura.io/jwt/claims")]
    pub hasura: HasuraClaims,
}

fn parse_token(option: Option<&str>) -> Result<Claims, ()> {
    let header = option.ok_or(())?;
    let mut iter = header.split(' ');

    let (bearer, token) = (iter.next().ok_or(())?, iter.next().ok_or(())?);
    (bearer != "Bearer").then_some(()).ok_or(())?;

    let secret = std::env::var("SECRET_KEY").map_err(|_| ())?;
    let key = DecodingKey::from_secret(secret.as_bytes());

    let data = decode(token, &key, &Validation::default()).map_err(|_| ())?;
    Ok(data.claims)
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Claims {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match parse_token(request.headers().get_one("Authorization")) {
            Ok(claims) => Outcome::Success(claims),
            Err(_) => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}
