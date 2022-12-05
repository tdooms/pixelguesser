#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Hasura error: {0:?}")]
    Hasura(#[from] hasura::Error),

    #[error("Serde error: {0:?}")]
    Serde(#[from] serde_json::Error),

    #[cfg(feature = "session")]
    #[error("Session error {0}")]
    Session(#[from] pixessions::Error),

    #[error("Error connecting to {0} at {1}")]
    UnreachableHost(&'static str, String),

    #[error("Received response from {0} with error status")]
    StatusCode(&'static str),

    #[error("Received invalid response from {0}")]
    InvalidResponse(&'static str),

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
