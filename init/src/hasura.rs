use api::GRAPHQL_ENDPOINT;
use hasura::{mutation, Delete, Insert};

pub async fn upload_quizzes() -> Vec<String> {
    let insert = Insert::new(quizzes);
    let inserted = mutation!(insert).token(bearer).send(GRAPHQL_ENDPOINT).await.unwrap();

    inserted.into_iter().map(|x| x.title).collect()
}

pub async fn delete_quizzes() -> Vec<String> {
    let delete = Delete::new();
    let deleted = mutation!(delete).token(token).send(GRAPHQL_ENDPOINT).await.unwrap();

    deleted.into_iter().map(|x| x.title).collect()
}
