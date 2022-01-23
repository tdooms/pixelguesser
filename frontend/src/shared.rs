use yew_router::prelude::*;

pub static SELF_ENDPOINT: &str = include_str!("keys/self-endpoint.in");
pub static IMAGE_ENDPOINT: &str = include_str!("keys/image-endpoint.in");
pub static IMAGE_PLACEHOLDER: &str = include_str!("keys/image-placeholder.in");
pub static SESSION_ENDPOINT: &str = include_str!("keys/session-endpoint.in");
pub static GRAPHQL_ENDPOINT: &str = include_str!("keys/graphql-endpoint.in");
pub static HASURA_SECRET: &str = include_str!("keys/hasura-secret.in");
pub static AUTH0_DOMAIN: &str = include_str!("keys/auth0_domain.in");
pub static AUTH0_CLIENT_ID: &str = include_str!("keys/auth0_client_id.in");

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/manage/:code")]
    Manage { code: String },
    #[at("/host/:quiz_id")]
    Host { quiz_id: u64 },
    #[at("/create")]
    Create,
    #[at("/update/:quiz_id")]
    Update { quiz_id: u64 },
    #[at("/test")]
    Test,
    #[at("/")]
    Overview,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("reqwasm error {0}")]
    Reqwasm(#[from] reqwasm::Error),

    #[error("websocket error {0}")]
    WebSocket(String),

    #[error("graphql error {0:?}")]
    Graphql(Vec<String>),

    #[error("local image already uploaded")]
    Reupload,

    #[error("file read error {0}")]
    FileError(gloo::file::FileReadError),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq)]
pub enum User {
    Loading,
    Anonymous,
    User(UserData),
}

impl Default for User {
    fn default() -> Self {
        Self::Loading
    }
}

#[derive(serde::Deserialize, Clone, Debug, PartialEq)]
pub struct UserData {
    pub nickname: String,
    pub name: String,
    pub picture: String,
    pub updated_at: String,
    pub email: String,
    pub email_verified: bool,
    pub sub: String,
}
