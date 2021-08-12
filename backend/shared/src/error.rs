#[derive(thiserror::Error, Debug, serde::Serialize, Clone)]
pub enum Error {
    #[error("The session with id {0} does not exist")]
    SessionDoesNotExist(u64),

    #[error("The player with name {0} does not exist")]
    PlayerDoesNotExist(String),

    #[error("A player with name {0} already exists")]
    DuplicatePlayerName(String),

    #[error("An internal error caused the creation of a new session to fail")]
    SessionCreationFailed,
}