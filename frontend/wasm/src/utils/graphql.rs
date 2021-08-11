use std::fmt::Display;

use serde::de::DeserializeOwned;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug)]
pub struct Paginate<T> {
    data: Vec<T>,
    before: Option<String>,
    after: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Query<T> {
    data: T,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Quizzes {
    quizzes: Paginate<Quiz>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Quiz {
    pub name: String,
    pub description: String,
    pub author: String,
    pub created: DateTime<Utc>,
    pub image_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FindQuiz {
    #[serde(rename(deserialize = "findQuizByID"))]
    quiz: QuizWithRounds,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuizWithRounds {
    #[serde(flatten)]
    quiz: Quiz,

    rounds: Paginate<Round>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Round {
    answer: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rounds {
    rounds: Paginate<Round>,
}

pub async fn exec_query<Call, Ret, Msg>(query: impl Display, callback: Call) -> Msg
    where Ret: DeserializeOwned,
          Call: FnOnce(Ret) -> Msg
{
    // shh this is mine
    const KEY: &str = "fnAEQQ8uCwACDLpj_kfilwMMyeItMBlcHTGn4Rb1";
    const URL: &str = "https://graphql.fauna.com/graphql";

    let body = format!("{{ \"query\":\"{}\" }}", query);

    let res: Query<T> = reqwest::Client::new().post(URL)
        .header("Authorization", format!("Bearer {}", KEY))
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    callback(res.data)
}

pub const OVERVIEW_QUERY: &str = "query overview { quizzes { data { name description author created image_url} } }";