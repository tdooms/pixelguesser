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

    #[error("Invalid update {0:?} on session {1:?}")]
    InvalidUpdate(Action, Session),

    #[error("Could not parse request")]
    FaultyRequest,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Request {
    Host,        // amount of rounds
    Manage(u64), // session id
    Update(Action, u64),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Response {
    Hosted(u64, Session),
    Managed(u64, Session),
    Updated(Session),
    Error(Error),
}
