use serde::{Deserialize, Serialize};

use crate::Session;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Request {
    Read(u64),
    Update(u64, Session),
    Check(u64),

    Create,
    Host(u64),
    Manage(u64)
}

pub enum Response {
    Read(Session),
    Updated(Session),
    Checked(Option<u64>),

    Created(u64),
    Hosted(u64),
    Managed(u64),

    Error(String),
}

