use std::fmt::Debug;

use reqwasm::http::{Method, Request};
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::{
    CreateQuizData, DeleteQuizData, DraftQuiz, DraftRound, Error, FullQuiz, Quiz, QuizData,
    QuizzesData, Resolution, SaveRoundsData, UpdateQuizData, User, GRAPHQL_ENDPOINT, QUIZ_FIELDS,
    ROUND_FIELDS,
};

pub enum Kind<'r> {
    Query(&'r str),
    Mutation(&'r str),
    Subscription(&'r str),
}

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
enum Response<T> {
    Data { data: T },
    Errors { errors: Vec<GraphqlError> },
}

#[derive(serde::Deserialize, Debug)]
pub struct GraphqlError {
    pub extensions: Value,
    pub message: String,
}

pub async fn exec<T: DeserializeOwned + Debug>(
    user: Option<User>,
    query: Kind<'_>,
) -> Result<T, Error> {
    let body = match query {
        Kind::Query(str) => format!("{{\"query\":\"query {{ {} }}\"}}", str),
        Kind::Mutation(str) => {
            format!("{{\"query\":\"mutation {{ {} }}\" }}", str)
        }
        Kind::Subscription(str) => format!("{{\"query\":\"subscription {{ {} }}\"}}", str),
    };

    log::debug!("request {}", body);

    let builder = Request::new(GRAPHQL_ENDPOINT)
        .method(Method::POST)
        .header("content-type", "application/json");

    let builder = match &user {
        Some(user) => builder.header("authorization", &user.token),
        None => builder,
    };

    let response = builder.body(body).send().await?.text().await?;

    log::debug!("response {}", response);

    match serde_json::from_str(&response)? {
        Response::Data { data } => Ok(data),
        Response::Errors { errors } => {
            Err(Error::Graphql(errors.into_iter().map(|x| x.message).collect()))
        }
    }
}

fn serialize_round(quiz_id: u64, index: usize, draft: &DraftRound) -> String {
    let image = draft
        .image
        .as_ref()
        .map(|x| format!(", image: \\\"{}\\\"", x.src(Resolution::Max)))
        .unwrap_or_default();

    let speed = draft.speed.as_ref().map(|x| format!(", speed: \\\"{}\\\"", x)).unwrap_or_default();

    format!(
        "quiz_id:\\\"{}\\\",index:\\\"{}\\\",answer:\\\"{}\\\",guesses:\\\"{}\\\",points:\\\"{}\\\"{}{}",
        quiz_id, index, draft.answer, draft.guesses as u8, draft.points as u8, speed, image
    )
}

fn serialize_quiz(user: &User, draft: &DraftQuiz) -> String {
    let image = draft
        .image
        .as_ref()
        .map(|x| format!(", image: \\\"{}\\\"", x.src(Resolution::Max)))
        .unwrap_or_default();

    format!(
        "title:\\\"{}\\\",description:\\\"{}\\\",creator_id:\\\"{}\\\",explanation:\\\"{}\\\"{}",
        draft.title, draft.description, user.sub, draft.explanation, image
    )
}

pub async fn quizzes(user: Option<User>) -> Result<Vec<Quiz>, Error> {
    let str = format!("quizzes {{ {} }}", QUIZ_FIELDS);

    let data: QuizzesData = exec(user, Kind::Query(&str)).await?;
    Ok(data.quizzes)
}

pub async fn full_quiz(user: Option<User>, quiz_id: u64) -> Result<FullQuiz, Error> {
    let str = format!(
        "quizzes_by_pk(id: {}) {{ {} }} rounds(where: {{quiz_id: {{ _eq: {} }} }}) {{ {} }}",
        quiz_id, QUIZ_FIELDS, quiz_id, ROUND_FIELDS
    );

    let data: QuizData = exec(user, Kind::Query(&str)).await?;
    Ok(FullQuiz { quiz: data.quizzes_by_pk, rounds: data.rounds })
}

pub async fn create_quiz(user: Option<User>, draft: DraftQuiz) -> Result<Option<Quiz>, Error> {
    let object = serialize_quiz(user.as_ref().ok_or(Error::NotLoggedIn)?, &draft);
    let str = format!("insert_quizzes_one(object: {{ {} }}) {{ id }}", object);

    let data: CreateQuizData = exec(user, Kind::Mutation(&str)).await?;
    Ok(data.insert_quizzes_one)
}

pub async fn update_quiz(
    user: Option<User>,
    id: u64,
    draft: DraftQuiz,
) -> Result<Option<Quiz>, Error> {
    let object = serialize_quiz(user.as_ref().ok_or(Error::NotLoggedIn)?, &draft);

    let str = format!(
        "update_quizzes_by_pk(_set: {{ {} }}, pk_columns: {{ id: \\\"{}\\\" }}) {{ {} }}",
        object, id, QUIZ_FIELDS
    );

    let data: UpdateQuizData = exec(user, Kind::Mutation(&str)).await?;
    Ok(data.update_quizzes_one)
}

pub async fn delete_quiz(user: User, quiz_id: u64) -> Result<Option<Quiz>, Error> {
    let rounds = format!(
        "delete_rounds(where: {{ quiz_id: {{ _eq: \\\"{}\\\" }} }} ) {{affected_rows}}",
        quiz_id
    );
    let quiz = format!("delete_quizzes_by_pk(id: \\\"{}\\\") {{ {} }}", quiz_id, QUIZ_FIELDS);
    let str = format!("{rounds} {quiz}");

    let data: DeleteQuizData = exec(Some(user), Kind::Mutation(&str)).await?;
    Ok(data.delete_quizzes_by_pk)
}

pub async fn save_rounds(
    user: User,
    quiz_id: u64,
    rounds: Vec<DraftRound>,
) -> Result<Vec<DraftRound>, Error> {
    let serialized: Vec<_> = rounds
        .iter()
        .enumerate()
        .map(|(idx, round)| serialize_round(quiz_id, idx, round))
        .collect();

    let objects = format!("[{{ {} }}]", serialized.join("},{"));
    let str = format!(
        "\
    delete_rounds(where: {{ quiz_id: {{ _eq: \\\"{}\\\" }} }} ) {{affected_rows}}\
    insert_rounds(objects: {}) {{ affected_rows }}",
        quiz_id, objects
    );

    let _: SaveRoundsData = exec(Some(user), Kind::Mutation(&str)).await?;
    Ok(rounds)
}
