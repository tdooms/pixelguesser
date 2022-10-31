use api::{Quiz, GRAPHQL_ENDPOINT};
use hasura::{mutation, Delete, Insert};

pub async fn upload_quizzes(
    mut quizzes: Vec<Quiz>,
    creator: String,
    bearer: String,
) -> Vec<String> {
    // set the creator id as admin for all quizzes
    for quiz in &mut quizzes {
        quiz.creator_id = Some(creator.clone());
    }

    // set the indices for each round
    for (index, round) in &mut quizzes.iter_mut().map(|q| q.rounds.iter_mut().enumerate()).flatten()
    {
        round.index = index as u64;
    }

    let insert = Insert::new(quizzes);
    let inserted = mutation!(insert).token(bearer).send(GRAPHQL_ENDPOINT).await.unwrap();

    inserted.into_iter().map(|x| x.title).collect()
}

pub async fn delete_quizzes(bearer: String) -> Vec<String> {
    let delete = Delete::new();
    let deleted = mutation!(delete).token(bearer).send(GRAPHQL_ENDPOINT).await.unwrap();

    deleted.into_iter().map(|x| x.title).collect()
}
