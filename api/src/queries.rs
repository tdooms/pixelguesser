use hasura::{
    delete::*, insert::*, mutation, query, query::*, update::*, Conditions, Eq, Ilike, Object,
};

use crate::{DraftQuiz, Error, Quiz, QuizPk, Result, Round, Tag, User, GRAPHQL_ENDPOINT};

pub async fn query_quizzes(user: Option<User>, rounds: bool) -> Result<Vec<Quiz>> {
    let returning = match rounds {
        false => Quiz::except(&[Quiz::rounds(Round::all())]),
        true => Quiz::all(),
    };
    let body = QueryBuilder::default().returning(returning).build().unwrap();
    Ok(query!(body).token(user.map(|x| x.token)).send(GRAPHQL_ENDPOINT).await?)
}

pub async fn search_quizzes(user: Option<User>, query: String, rounds: bool) -> Result<Vec<Quiz>> {
    let condition = Ilike(format!("%{}%", query));
    let conditions = Conditions::single(Quiz::title(), condition);

    let returning = match rounds {
        false => Quiz::except(&[Quiz::rounds(Round::all())]),
        true => Quiz::all(),
    };

    let body =
        QueryBuilder::default().conditions(vec![conditions]).returning(returning).build().unwrap();

    Ok(query!(body).token(user.map(|x| x.token)).send(GRAPHQL_ENDPOINT).await?)
}

pub async fn query_quiz(user: Option<User>, quiz_id: u32) -> Result<Quiz> {
    let first = QueryByPkBuilder::default()
        .pk(QuizPk { id: quiz_id.into() })
        .returning(Quiz::all())
        .build()
        .unwrap();

    let token = user.map(|x| x.token);
    let fut = query!(first).token(token).send(GRAPHQL_ENDPOINT);
    let mut res = fut.await?.ok_or(Error::EmptyResponse)?;

    res.rounds.sort_by_key(|x| x.index);

    Ok(res)
}

pub async fn create_quiz(user: User, draft: DraftQuiz) -> Result<Quiz> {
    let first = InsertOneBuilder::default().returning(Quiz::all()).object(draft).build().unwrap();

    let fut = mutation!(first).token(Some(user.token)).send(GRAPHQL_ENDPOINT);
    fut.await?.ok_or(Error::EmptyResponse)
}

pub async fn update_quiz(user: User, quiz_id: u32, mut draft: DraftQuiz) -> Result<Quiz> {
    let tags = std::mem::take(&mut draft.tags.data);
    let tags: Vec<_> = tags.into_iter().map(|x| Tag { value: x.value.clone(), quiz_id }).collect();

    let rounds = std::mem::take(&mut draft.rounds.data);
    let rounds: Vec<_> = rounds
        .into_iter()
        .enumerate()
        .map(|(index, draft)| Round::from_draft(draft, quiz_id, index as u32))
        .collect();

    let first = UpdateByPkBuilder::default()
        .pk(QuizPk { id: quiz_id })
        .returning(Quiz::all())
        .set(draft)
        .build()
        .unwrap();

    let condition = Eq(quiz_id.to_string());
    let second = DeleteBuilder::default()
        .returning(Tag::all())
        .conditions(vec![Conditions::single(Tag::quiz_id(), condition)])
        .build()
        .unwrap();

    let third = InsertBuilder::default().returning(Tag::all()).objects(tags).build().unwrap();

    let condition = Eq(quiz_id.to_string());
    let fourth = DeleteBuilder::default()
        .returning(Round::all())
        .conditions(vec![Conditions::single(Round::quiz_id(), condition)])
        .build()
        .unwrap();

    let fifth = InsertBuilder::default().returning(Round::all()).objects(rounds).build().unwrap();

    let token = Some(user.token.clone());
    let _ = mutation!(fourth, fifth).token(token.clone()).send(GRAPHQL_ENDPOINT).await?;

    let fut = mutation!(first, second, third).token(token).send(GRAPHQL_ENDPOINT);
    fut.await?.0.ok_or(Error::EmptyResponse)
}

pub async fn delete_quiz(user: User, quiz_id: u32) -> Result<Quiz> {
    let first = DeleteByPkBuilder::default()
        .pk(QuizPk { id: quiz_id })
        .returning(Quiz::all())
        .build()
        .unwrap();

    let token = Some(user.token.clone());
    mutation!(first).token(token).send(GRAPHQL_ENDPOINT).await?.ok_or(Error::EmptyResponse)
}
