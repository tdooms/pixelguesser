use crate::{Error, Result, GRAPHQL_ENDPOINT};
use hasura::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Default, Hasura)]
#[hasura(table = "users")]
pub struct User {
    #[hasura(pk = "String")]
    pub id: String,

    pub nickname: String,
    pub picture: String,
    pub email: String,
    pub email_verified: bool,
}

pub async fn query_user(token: Option<String>, user_id: String) -> Result<Option<User>> {
    let body = QueryByPk::new(UserPk { id: user_id });
    Ok(query!(body).token(token).send(GRAPHQL_ENDPOINT).await?)
}

pub async fn create_user(token: Option<String>, user: User) -> Result<User> {
    let body = InsertOne::new(user);
    mutation!(body).token(token).send(GRAPHQL_ENDPOINT).await?.ok_or(Error::EmptyResponse)
}
