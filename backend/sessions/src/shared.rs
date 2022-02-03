use crate::session::{Action, Session};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Clone, Debug, Serialize, Deserialize)]
pub enum Error {
    #[error("Could not join session {0}")]
    UnableToJoin(u64),

    #[error("Could not create a new session due to an internal server error")]
    UnableToCreate,

    #[error("Could not join as manager in session {0}")]
    UnableToManage(u64),

    #[error("The host for this session disconnected")]
    HostDisconnected,

    #[error("Invalid update {0:?} on session {1:?}")]
    InvalidUpdate(Action, Session),

    #[error("You are not allowed to perform this operation")]
    IllegalRequest,

    #[error("Could not parse request")]
    FaultyRequest,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Request {
    Host,        // amount of rounds
    Manage(u64), // session id
    Update(Action, usize),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Response {
    pub id: u64,
    pub managed: bool,
    pub session: Session,
}
