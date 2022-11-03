use std::rc::Rc;

use chrono::{DateTime, Utc};
use hasura::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{Error, Image, Result, Round, User, GRAPHQL_ENDPOINT};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hasura)]
#[hasura(table = "tags")]
pub struct Tag {
    #[hasura(pk = "u64")]
    pub quiz_id: Option<u64>,

    #[hasura(pk = "String")]
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Validate, Hasura, Default)]
#[hasura(table = "quizzes")]
pub struct Quiz {
    #[hasura(pk = "u64")]
    pub id: Option<u64>,

    pub public: bool,
    pub complete: bool,

    #[validate(length(min = 1, message = "Quiz must have a title."))]
    #[validate(length(max = 32, message = "Title cannot exceed 32 characters."))]
    pub title: String,

    #[validate(length(max = 64, message = "Description cannot exceed 64 characters."))]
    pub description: String,

    #[validate(length(max = 128, message = "Explanation cannot exceed 128 characters."))]
    pub explanation: String,

    pub created_at: Option<DateTime<Utc>>,
    pub creator_id: Option<String>,

    #[serde(default)]
    pub image: Image,

    #[hasura(relation = "User")]
    #[serde(with = "relation")]
    #[serde(default)]
    pub creator: Option<User>,

    #[hasura(relation = "Tag")]
    #[serde(with = "relation")]
    #[serde(default)]
    pub tags: Vec<Tag>,

    #[hasura(relation = "Round")]
    #[serde(with = "relation")]
    #[serde(default)]
    pub rounds: Vec<Round>,
}

impl Quiz {
    pub async fn query_many(
        token: Option<String>,
        session: Option<u64>,
        rounds: bool,
    ) -> Result<Vec<Quiz>> {
        let returning = match rounds {
            false => Quiz::except(&[Quiz::rounds(Round::all())]),
            true => Quiz::all(),
        };

        let key = "x-hasura-session-id";
        let value = session.map(|x| x.to_string()).unwrap_or_default();

        let body = Query::new().returning(returning);
        Ok(query!(body).token(token).header(key, value).send(GRAPHQL_ENDPOINT).await?)
    }

    pub async fn query_one(
        token: Option<String>,
        quiz_id: u64,
        session: Option<u64>,
    ) -> Result<Quiz> {
        let body = QueryByPk::new(QuizPk { id: quiz_id.into() });

        let key = "x-hasura-session-id";
        let value = session.map(|x| x.to_string()).unwrap_or_default();

        let fut = query!(body).token(token).header(key, value).send(GRAPHQL_ENDPOINT);
        let mut res: Quiz = fut.await?.ok_or(Error::EmptyResponse)?;

        res.rounds.sort_by_key(|x| x.index);

        Ok(res)
    }

    pub async fn search(token: Option<String>, query: String) -> Result<Vec<Quiz>> {
        let conditions = Conditions::single(Quiz::title(), Ilike(format!("%{}%", query)));
        let returning = Quiz::except(&[Quiz::rounds(Round::all())]);

        let body = Query::new().conditions(conditions).returning(returning);
        Ok(query!(body).token(token).send(GRAPHQL_ENDPOINT).await?)
    }

    pub async fn create(token: String, quiz: Rc<Quiz>) -> Result<Quiz> {
        let body = InsertOne::new(quiz.as_ref());
        let fut = mutation!(body).token(Some(token)).send(GRAPHQL_ENDPOINT);

        fut.await?.ok_or(Error::EmptyResponse)
    }

    pub async fn update(token: String, quiz: Rc<Quiz>) -> Result<Quiz> {
        let id = quiz.id.unwrap();

        let conditions = Conditions::single(Tag::quiz_id(), Eq(id.to_string()));
        let tags = Delete::new().conditions(conditions);

        let conditions = Conditions::single(Round::quiz_id(), Eq(id.to_string()));
        let rounds = Delete::new().conditions(conditions);

        let quiz = UpdateByPk::new(QuizPk { id }, quiz.as_ref());

        let res = mutation!(tags, rounds, quiz).token(Some(token)).send(GRAPHQL_ENDPOINT).await?;
        res.2.ok_or(Error::EmptyResponse)
    }

    pub async fn delete(token: String, quiz: Rc<Quiz>) -> Result<Quiz> {
        let first = DeleteByPk::new(QuizPk { id: quiz.id.unwrap() });
        mutation!(first)
            .token(Some(token))
            .send(GRAPHQL_ENDPOINT)
            .await?
            .ok_or(Error::EmptyResponse)
    }
}
