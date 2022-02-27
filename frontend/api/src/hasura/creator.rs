use crate::auth0::User;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Creator {
    pub id: String,
    pub name: String,
}

impl From<User> for Creator {
    fn from(user: User) -> Self {
        Self { id: user.sub, name: user.nickname }
    }
}
