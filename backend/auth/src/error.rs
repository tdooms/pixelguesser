use rocket::http::Status;
use rocket::response::Responder;
use rocket::{response, Request};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("{0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("{0}")]
    SystemTime(#[from] std::time::SystemTimeError),

    #[error("{0}")]
    VarError(#[from] std::env::VarError),

    #[error("invalid role")]
    InvalidRole,

    #[error("malformed jwt")]
    MalformedJwt,

    #[error("authorization failed")]
    AuthorizationFailed,

    #[error("no authorization header")]
    AuthorizationHeader,

    #[error("no authorization header")]
    NoSecretKey,
}

impl<'r, 'o: 'r> Responder<'r, 'o> for Error {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        println!("{:?}", self);
        Status::InternalServerError.respond_to(req)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
