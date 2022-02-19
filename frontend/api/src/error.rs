use cynic::DecodeError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Request error: {0}")]
    Request(#[from] reqwasm::Error),

    #[error("Decode error {0}")]
    Decode(#[from] DecodeError),

    #[error("Encode error {0}")]
    Serde(#[from] cynic::serde_json::Error),

    #[error("Session error {0}")]
    Session(#[from] sessions::Error),

    #[error("Connection closed")]
    WsClosed,

    #[error("Connection error")]
    WsError,

    #[error("Empty Graphql response")]
    Empty,
}
