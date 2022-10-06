use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "user")]
    User = 0,
    #[serde(rename = "moderator")]
    Moderator = 1,
    #[serde(rename = "admin")]
    Admin = 2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HasuraClaims {
    #[serde(rename = "x-hasura-default-role")]
    pub default_role: Role,

    #[serde(rename = "x-hasura-allowed-roles")]
    pub allowed_roles: Vec<Role>,

    #[serde(rename = "x-hasura-user-id")]
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: u64,

    pub role: Role,

    #[serde(rename = "https://hasura.io/jwt/claims")]
    pub hasura: HasuraClaims,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tokens {
    pub bearer: String,
    pub refresh: String,
    pub id: String,
    pub expiry: u64,
}

#[cfg(any(feature = "verify", feature = "server"))]
mod verify {
    use jsonwebtoken::{decode, DecodingKey, Validation};
    use rocket::http::Status;
    use rocket::request::{FromRequest, Outcome};
    use rocket::*;

    use crate::Claims;

    fn parse_token(option: Option<&str>) -> Result<Claims, ()> {
        let header = option.ok_or(())?;
        let mut iter = header.split(' ');

        let (prefix, bearer) = (iter.next().ok_or(())?, iter.next().ok_or(())?);
        (prefix == "Bearer").then_some(()).ok_or(())?;

        let secret = std::env::var("AUTH_SECRET").map_err(|_| ())?;
        let key = DecodingKey::from_secret(secret.as_bytes());

        let data = decode(bearer, &key, &Validation::default()).map_err(|_| ())?;
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
}
