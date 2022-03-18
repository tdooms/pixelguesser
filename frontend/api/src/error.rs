#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Request error: {0}")]
    Request(#[from] reqwasm::Error),

    #[error("Graphql error: {0:?}")]
    Graphql(Vec<String>),

    #[error("Serde error: {0:?}")]
    Serde(#[from] serde_json::Error),

    #[error("Session error {0}")]
    Session(#[from] sessions::Error),

    #[error("Connection closed")]
    WsClosed,

    #[error("Connection error")]
    WsError,

    #[error("Empty Graphql response")]
    Empty,

    #[error("Not logged in")]
    NotLoggedIn,
}
