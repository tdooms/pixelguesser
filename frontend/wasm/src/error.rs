
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("reqwasm error {0}")]
    Reqwasm(#[from] reqwasm::Error),

    #[error("websocket error {0}")]
    WebSocket(String),

    #[error("graphql error {0:?}")]
    Graphql(Vec<String>),
}