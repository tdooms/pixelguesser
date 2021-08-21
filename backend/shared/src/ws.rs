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
    UnknownRequest
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Request {
    Read(u64),
    Update(u64, Session),

    Create,
    Host(u64),
    UnHost(u64),
    Manage(u64),
    UnManage(u64)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Response {
    Read(Session),
    Updated(Session),
    Created(u64),
    Error(Error),
}

