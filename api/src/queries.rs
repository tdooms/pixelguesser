use hasura::{
    delete::*, insert::*, mutation, query, query::*, update::*, Conditions, Eq, Ilike, Object,
};

use crate::{
    DraftQuiz, DraftRound, Error, FullQuiz, Quiz, QuizPk, Result, Round, Tag, User,
    GRAPHQL_ENDPOINT,
};

pub async fn quizzes(user: Option<User>) -> Result<Vec<Quiz>> {
    let body: Query<Quiz> = QueryBuilder::default().returning(Quiz::all()).build().unwrap();
    Ok(query!(body).token(user.map(|x| x.token)).send(GRAPHQL_ENDPOINT).await?)
}

pub async fn search_quizzes(user: Option<User>, query: String) -> Result<Vec<Quiz>> {
    let condition = Ilike(format!("%{}%", query));
    let conditions = Conditions::single(Quiz::title(), condition);

    let body = QueryBuilder::default()
        .conditions(vec![conditions])
        .returning(Quiz::all())
        .build()
        .unwrap();

    Ok(query!(body).token(user.map(|x| x.token)).send(GRAPHQL_ENDPOINT).await?)
}

pub async fn full_quiz(user: Option<User>, quiz_id: u32) -> Result<FullQuiz> {
    let condition = Eq(quiz_id.to_string());
    let conditions = Conditions::single(Round::quiz_id(), condition);

    let quiz = QueryByPkBuilder::default()
        .pk(QuizPk { id: quiz_id.into() })
        .returning(Quiz::all())
        .build()
        .unwrap();

    let rounds: Query<Round> = QueryBuilder::default()
        .conditions(vec![conditions])
        .returning(Round::all())
        .build()
        .unwrap();

    let token = user.map(|x| x.token);
    let (quiz, mut rounds) = query!(quiz, rounds).token(token).send(GRAPHQL_ENDPOINT).await?;
    rounds.sort_by_key(|x| x.index);

    Ok(FullQuiz { quiz: quiz.ok_or(Error::EmptyResponse)?, rounds })
}

pub async fn create_quiz(user: User, draft: DraftQuiz) -> Result<Option<Quiz>> {
    let first = InsertOneBuilder::default().returning(Quiz::all()).object(draft).build().unwrap();
    Ok(mutation!(first).token(Some(user.token)).send(GRAPHQL_ENDPOINT).await?)
}

pub async fn update_quiz(user: User, quiz_id: u32, mut draft: DraftQuiz) -> Result<Option<Quiz>> {
    let condition = Eq(quiz_id.to_string());
    let conditions = Conditions::single(Tag::quiz_id(), condition);

    let tags = std::mem::take(&mut draft.tags.data);
    let tags: Vec<_> = tags.into_iter().map(|x| Tag { value: x.value.clone(), quiz_id }).collect();

    let first = UpdateByPkBuilder::default()
        .pk(QuizPk { id: quiz_id })
        .returning(Quiz::all())
        .set(draft)
        .build()
        .unwrap();

    let second = DeleteBuilder::default()
        .returning(Tag::all())
        .conditions(vec![conditions])
        .build()
        .unwrap();

    let third = InsertBuilder::default().returning(Tag::all()).objects(tags).build().unwrap();

    let (res, _, _) =
        mutation!(first, second, third).token(Some(user.token)).send(GRAPHQL_ENDPOINT).await?;

    Ok(res)
}

pub async fn delete_quiz(user: User, quiz_id: u32) -> Result<Option<Quiz>> {
    let first = DeleteByPkBuilder::default()
        .pk(QuizPk { id: quiz_id })
        .returning(Quiz::all())
        .build()
        .unwrap();

    let condition = Eq(quiz_id.to_string());
    let conditions = Conditions::single(Round::quiz_id(), condition);

    let second = DeleteBuilder::default()
        .returning(Round::all())
        .conditions(vec![conditions])
        .build()
        .unwrap();

    // TODO: maybe also return rounds
    let (res, _) = mutation!(first, second).token(Some(user.token)).send(GRAPHQL_ENDPOINT).await?;
    Ok(res)
}

pub async fn save_rounds(user: User, quiz_id: u32, rounds: Vec<DraftRound>) -> Result<Vec<Round>> {
    let condition = Eq(quiz_id.to_string());
    let conditions = Conditions::single(Round::quiz_id(), condition);

    let delete = DeleteBuilder::default()
        .returning(Round::all())
        .conditions(vec![conditions])
        .build()
        .unwrap();

    let objects: Vec<_> = rounds
        .into_iter()
        .enumerate()
        .map(|(idx, draft)| Round::from_draft(draft, quiz_id, idx as u32))
        .collect();

    let insert = InsertBuilder::default().returning(Round::all()).objects(objects).build().unwrap();

    // TODO: set the quiz to complete
    // let update = UpdateByPk::default().pk(Quiz::Pk { id: quiz_id} ).set()
    let (_, res) = mutation!(delete, insert).token(Some(user.token)).send(GRAPHQL_ENDPOINT).await?;
    Ok(res)
}
