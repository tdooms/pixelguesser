use crate::{Error, Result, GRAPHQL_ENDPOINT};
use hasura::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Clone, Validate, Default, Debug, PartialEq)]
pub struct Credentials {
    #[validate(email(message = "Must be a valid email address"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be longer than 8 characters"))]
    #[validate(length(max = 32, message = "Password must be shorter than 32 characters"))]
    pub password: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Default, Hasura)]
#[hasura(table = "users")]
pub struct User {
    #[hasura(pk = "String")]
    pub id: Option<String>,

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
