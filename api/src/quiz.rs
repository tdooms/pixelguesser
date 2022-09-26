use crate::{Error, Image, Result, Round, User, GRAPHQL_ENDPOINT};
use chrono::{DateTime, Utc};
use hasura::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

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

    #[validate(length(max = 64, message = "Description cannot exceed 32 characters."))]
    pub description: String,

    #[validate(length(max = 128, message = "Explanation cannot exceed 32 characters."))]
    pub explanation: String,

    pub created_at: Option<DateTime<Utc>>,
    pub creator_id: Option<String>,

    #[serde(default)]
    pub image: Image,

    #[hasura(relation = "User")]
    #[serde(with = "relation")]
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

pub async fn query_quizzes(token: Option<String>, rounds: bool) -> Result<Vec<Quiz>> {
    let returning = match rounds {
        false => Quiz::except(&[Quiz::rounds(Round::all())]),
        true => Quiz::all(),
    };
    let body = Query::new().returning(returning);
    Ok(query!(body).token(token).send(GRAPHQL_ENDPOINT).await?)
}

pub async fn search_quizzes(token: Option<String>, query: String) -> Result<Vec<Quiz>> {
    let conditions = Conditions::single(Quiz::title(), Ilike(format!("%{}%", query)));
    let returning = Quiz::except(&[Quiz::rounds(Round::all())]);

    let body = Query::new().conditions(conditions).returning(returning);
    Ok(query!(body).token(token).send(GRAPHQL_ENDPOINT).await?)
}

pub async fn query_quiz(token: Option<String>, quiz_id: u64) -> Result<Quiz> {
    let body = QueryByPk::new(QuizPk { id: quiz_id.into() });

    let fut = query!(body).token(token).send(GRAPHQL_ENDPOINT);
    let mut res: Quiz = fut.await?.ok_or(Error::EmptyResponse)?;

    res.rounds.sort_by_key(|x| x.index);

    Ok(res)
}

pub async fn create_quiz(token: String, quiz: Quiz) -> Result<Quiz> {
    let body = InsertOne::new(quiz);
    let fut = mutation!(body).token(Some(token)).send(GRAPHQL_ENDPOINT);

    fut.await?.ok_or(Error::EmptyResponse)
}

pub async fn update_quiz(token: String, quiz: Quiz) -> Result<Quiz> {
    let id = quiz.id.unwrap();

    let conditions = Conditions::single(Tag::quiz_id(), Eq(id.to_string()));
    let tags = Delete::new().conditions(conditions);

    let conditions = Conditions::single(Round::quiz_id(), Eq(id.to_string()));
    let rounds = Delete::new().conditions(conditions);

    let quiz = UpdateByPk::new(QuizPk { id }, quiz);

    let res = mutation!(tags, rounds, quiz).token(Some(token)).send(GRAPHQL_ENDPOINT).await?;
    res.2.ok_or(Error::EmptyResponse)
}

pub async fn delete_quiz(token: String, quiz_id: u64) -> Result<Quiz> {
    let first = DeleteByPk::new(QuizPk { id: quiz_id });
    mutation!(first).token(Some(token)).send(GRAPHQL_ENDPOINT).await?.ok_or(Error::EmptyResponse)
}
