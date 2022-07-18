use crate::auth0::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, hasura::Object, hasura::Pk)]
#[object(name = "users", pk = "id")]
pub struct Creator {
    pub id: String,
    pub name: String,
}

impl From<User> for Creator {
    fn from(user: User) -> Self {
        Self { id: user.sub, name: user.nickname }
    }
}
