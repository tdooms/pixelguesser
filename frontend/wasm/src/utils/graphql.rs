use serde::de::DeserializeOwned;
use std::fmt::Display;

pub async fn exec_query<Call, Ret, Msg>(query: impl Display, callback: Call) -> Msg
    where Ret: DeserializeOwned,
          Call: FnOnce(Ret) -> Msg
{
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