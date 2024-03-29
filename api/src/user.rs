use chrono::{DateTime, Utc};
use std::rc::Rc;

use hasura::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{Error, Result, HASURA_ENDPOINT};

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
    pub user_id: Option<u64>,

    pub nickname: String,

    // #[serde(default)]
    // pub image: Image,
    pub last_seen: Option<DateTime<Utc>>,
    pub email: String,
    pub verified: bool,
}

impl User {
    pub async fn query_one(token: Option<String>, user_id: String) -> Result<Option<User>> {
        let body = QueryByPk::new(UserPk { user_id });
        Ok(query!(body).token(token).send(HASURA_ENDPOINT).await?.parse()?)
    }

    pub async fn create(token: Option<String>, user: Rc<User>) -> Result<User> {
        let body = InsertOne::new(user.as_ref());

        mutation!(body)
            .token(token)
            .send(HASURA_ENDPOINT)
            .await?
            .parse()?
            .ok_or(Error::EmptyResponse)
    }
}
