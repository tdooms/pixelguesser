use sqlx::{Error, PgPool};

use api::{Quiz, Round};

// FIXME: propagate errors
pub async fn rounds(quiz_id: i64, pool: PgPool) -> Result<Vec<Round>, Error> {
    sqlx::query_as!(Round, "select * from rounds where quiz_id = $1", quiz_id)
        .fetch_all(&pool)
        .await
}

pub async fn quiz(quiz_id: i64, pool: PgPool) -> Result<Quiz, Error> {
    sqlx::query_as!(Quiz, "select * from quizzes where quiz_id = $1", quiz_id)
        .fetch_one(&pool)
        .await
}

pub async fn quizzes(pool: PgPool) -> Result<Vec<Quiz>, Error> {
    sqlx::query_as!(Quiz, "select * from quizzes")
        .fetch_all(&pool)
        .await
}

pub async fn quiz_and_rounds(quiz_id: i64, pool: PgPool) -> Result<(Quiz, Vec<Round>), Error> {
    // FIXME: could this be better with built-in function?
    match tokio::join!(quiz(quiz_id, pool.clone()), rounds(quiz_id, pool)) {
        (Ok(x), Ok(y)) => Ok((x, y)),
        (Err(x), _) => Err(x),
        (_, Err(y)) => Err(y),
    }
}
