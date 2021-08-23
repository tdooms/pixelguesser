use serde::{Deserialize, Serialize};

use crate::Session;

#[derive(thiserror::Error, Clone, Debug, Serialize, Deserialize)]
pub enum Error {
    #[error("The session with id {0} does not exist")]
    SessionDoesNotExist(u64),

    #[error("Could not create a new session due to an internal server error")]
    UnableToCreate,

    #[error("Could not host session")]
    UnableToHost(u64),

    #[error("Could not manage session")]
    UnableToManage(u64),

    #[error("Could not parse request")]
    UnknownRequest,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Request {
    Read { session_id: u64 },
    Update { session_id: u64, session: Session },

    Create { quiz_id: u64 },
    Host { session_id: u64 },
    Manage { session_id: u64 },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Response {
    Read(Session),
    Updated(Session),
    Created(u64, Session),
    Error(Error),
}
