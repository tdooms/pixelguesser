#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[cfg(feature = "wasm")]
    #[error("Request error: {0}")]
    Request(#[from] gloo_net::Error),

    #[cfg(feature = "native")]
    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Hasura error: {0:?}")]
    Hasura(#[from] hasura::Error),

    #[error("Serde error: {0:?}")]
    Serde(#[from] serde_json::Error),

    #[error("Session error {0}")]
    Session(#[from] sessions::Error),

    #[error("invalid session id")]
    InvalidSession,

    #[error("Image upload failed")]
    ImageUpload,

    #[error("Could not find image at path {0}")]
    ImageRead(String),

    #[error("Websocket connection failure")]
    WsConnection,

    #[error("Websocket connection closed")]
    WsClosed,

    #[error("Websocket connection closed")]
    WsFailure,

    #[error("Websocket received bytes instead of text")]
    WsBytes,

    #[error("Missing Graphql response")]
    EmptyResponse,
}

pub type Result<T> = std::result::Result<T, Error>;
