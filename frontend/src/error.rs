#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("reqwasm error {0}")]
    Reqwasm(#[from] reqwasm::Error),

    #[error("websocket error {0}")]
    WebSocket(String),

    #[error("graphql error {0:?}")]
    Graphql(Vec<String>),

    #[error("local image already uploaded")]
    Reupload,

    #[error("file read error {0}")]
    FileError(gloo::file::FileReadError),
}

pub type Result<T> = std::result::Result<T, Error>;
