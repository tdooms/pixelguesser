use hasura::{
    delete::*, insert::*, mutation, query, query::*, update::*, Conditions, Eq, Ilike, Object,
};

use crate::{
    DraftQuiz, DraftRound, Error, FullQuiz, Quiz, QuizPk, Result, Round, User, GRAPHQL_ENDPOINT,
};

pub async fn quizzes(user: Option<User>) -> Result<Vec<Quiz>> {
    let body: Query<Quiz> = QueryBuilder::default().returning(Quiz::all()).build().unwrap();
    Ok(query!(body).token(user.map(|x| x.token)).send(GRAPHQL_ENDPOINT).await?)
}

pub async fn search_quizzes(user: Option<User>, query: String) -> Result<Vec<Quiz>> {
    let condition = Ilike(format!("\\\"%{}%\\\"", query));
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

    let (quiz, rounds) =
        query!(quiz, rounds).token(user.map(|x| x.token)).send(GRAPHQL_ENDPOINT).await?;
    Ok(FullQuiz { quiz: quiz.ok_or(Error::Empty)?, rounds })
}

pub async fn create_quiz(user: User, draft: DraftQuiz) -> Result<Option<Quiz>> {
    let body = InsertOneBuilder::default().returning(Quiz::all()).object(draft).build().unwrap();
    Ok(mutation!(body).token(Some(user.token)).send(GRAPHQL_ENDPOINT).await?)
}

pub async fn update_quiz(user: User, id: u32, draft: DraftQuiz) -> Result<Option<Quiz>> {
    let body = UpdateByPkBuilder::default()
        .pk(QuizPk { id })
        .returning(Quiz::all())
        .set(draft)
        .build()
        .unwrap();

    Ok(mutation!(body).token(Some(user.token)).send(GRAPHQL_ENDPOINT).await?)
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
