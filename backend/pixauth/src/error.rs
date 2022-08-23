use rocket::response::Responder;

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
}

impl<'r, 'o: 'r> Responder<'r, 'o> for Error {
    fn respond_to(self, req: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
        println!("{:?}", self);
        rocket::http::Status::InternalServerError.respond_to(req)
    }
}
