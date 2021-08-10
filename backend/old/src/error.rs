use tokio::sync::mpsc;
use warp::ws::Message;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("The request session with id {0} does not exist (anymore)")]
    SessionDoesNotExist(u64),

    #[error("The request player with id {0} does not exist (anymore)")]
    PlayerDoesNotExist(u64),

    #[error("An internal error caused the creation of a new session to fail")]
    SessionCreationFailed,

    #[error("Could not deserialize request: {0}")]
    DeserializeError(#[from] serde_json::Error),

    #[error("A websocket error caused the response {0} to fail")]
    WsError(#[from] mpsc::error::SendError<Result<Message, warp::Error>>),
}
