use crate::Session;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Request {
    Read(u64),
    Update(Session),
    Check(u64),
    Create,
}

pub enum Response {
    Read(Session),
    Updated(Session),
    Checked(Option<u64>),
    Created
}

