use super::schema;
use crate::auth0::User;
use cynic::QueryFragment;
use serde::Deserialize;

#[derive(QueryFragment, Deserialize, Debug, Clone, PartialEq)]
#[cynic(schema_path = "schema.gql", query_module = "schema", graphql_type = "Users")]
pub struct Creator {
    pub id: String,
    pub name: String,
}

impl From<User> for Creator {
    fn from(user: User) -> Self {
        Self { id: user.sub, name: user.nickname }
    }
}
