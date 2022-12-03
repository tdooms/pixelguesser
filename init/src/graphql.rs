use api::{Quiz, GRAPHQL_ENDPOINT};
use hasura::{mutation, Delete, Insert};

pub async fn upload_quizzes(quizzes: &mut [Quiz], creator: u64, bearer: String) -> Vec<String> {
    // set the creator id as admin for all quizzes
    for quiz in quizzes.iter_mut() {
        quiz.creator_id = Some(creator.clone());
    }

    // set the indices for each round
    for (index, round) in &mut quizzes.iter_mut().map(|q| q.rounds.iter_mut().enumerate()).flatten()
    {
        round.round_index = index as u64;
    }

    let insert = Insert::new(&quizzes);
    let inserted = mutation!(insert).token(bearer).send(GRAPHQL_ENDPOINT).await.unwrap();

    println!("{inserted}");
    inserted.parse().unwrap().into_iter().map(|x| x.title).collect()
}

pub async fn delete_quizzes(bearer: String) -> Vec<String> {
    println!("{GRAPHQL_ENDPOINT}");
    let delete: Delete<Quiz> = Delete::new();
    let deleted = mutation!(delete).token(bearer).send(GRAPHQL_ENDPOINT).await.unwrap();

    println!("{deleted}");
    deleted.parse().unwrap().into_iter().map(|x| x.title).collect()
}
