use hasura::{mutation, Delete, Insert};

use api::{Quiz, Result, GRAPHQL_ENDPOINT};

pub async fn upload_quizzes(quizzes: &mut [Quiz], creator: u64, bearer: String) -> Result<()> {
    // set the creator id as admin for all quizzes
    for quiz in quizzes.iter_mut() {
        quiz.creator_id = Some(creator.clone());
    }

    // set the indices for each round
    for (index, round) in quizzes.iter_mut().map(|q| q.rounds.iter_mut().enumerate()).flatten() {
        round.round_index = index as u64;
    }

    let request = Insert::new(&quizzes);
    let response = mutation!(request).token(bearer).send(GRAPHQL_ENDPOINT).await?;

    let info: Vec<_> = response.parse()?.into_iter().map(|x| x.title).collect();
    tracing::info!("Uploaded quizzes: {info:?}");

    Ok(())
}

pub async fn delete_quizzes(bearer: String) -> Result<()> {
    let request: Delete<Quiz> = Delete::new();
    let response = mutation!(request).token(bearer).send(GRAPHQL_ENDPOINT).await?;

    let info: Vec<_> = response.parse()?.into_iter().map(|x| x.title).collect();
    tracing::info!("Deleted quizzes: {info:?}");

    Ok(())
}
