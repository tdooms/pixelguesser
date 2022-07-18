use std::num::ParseIntError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Request error: {0}")]
    Request(#[from] reqwasm::Error),

    #[error("Hasura error: {0:?}")]
    Hasura(#[from] hasura::Error),

    #[error("Serde error: {0:?}")]
    Serde(#[from] serde_json::Error),

    #[error("Session error {0}")]
    Session(#[from] sessions::Error),

    #[error("Image upload failed")]
    Upload,

    #[error("Connection closed")]
    WsClosed,

    #[error("Connection error")]
    WsError,

    #[error("Empty Graphql response")]
    Empty,

    #[error("Not logged in")]
    NotLoggedIn,

    #[error("Invalid session id")]
    InvalidSession(#[from] ParseIntError),
}

pub type Result<T> = std::result::Result<T, Error>;
