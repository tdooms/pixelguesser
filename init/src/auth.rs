use api::{Result, User, GRAPHQL_ENDPOINT};
use hasura::{mutation, Delete, InsertOne};

pub async fn upload_user(user: User, bearer: String) -> Result<()> {
    let body = InsertOne::new(&user);
    let _ = mutation!(body).token(Some(bearer.clone())).send(GRAPHQL_ENDPOINT).await?;
    Ok(())
}

pub async fn delete_user(token: String) -> Result<()> {
    let body: Delete<User> = Delete::new();
    let _ = mutation!(body).token(Some(token)).send(GRAPHQL_ENDPOINT).await?;
    Ok(())
}
