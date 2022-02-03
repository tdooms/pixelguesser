use std::fmt::{Display, Formatter};
use yew_router::prelude::*;

pub static SELF_ENDPOINT: &str = include_str!("keys/self-endpoint.in");
pub static IMAGE_ENDPOINT: &str = include_str!("keys/image-endpoint.in");
pub static IMAGE_PLACEHOLDER: &str = include_str!("keys/image-placeholder.in");
pub static SESSION_ENDPOINT: &str = include_str!("keys/session-endpoint.in");
pub static GRAPHQL_ENDPOINT: &str = include_str!("keys/graphql-endpoint.in");
pub static AUTH0_DOMAIN: &str = include_str!("keys/auth0-domain.in");
pub static AUTH0_CLIENT_ID: &str = include_str!("keys/auth0-client-id.in");

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
    #[error("Request error: {0}")]
    Request(#[from] reqwasm::Error),

    #[error("Session error {0}")]
    Session(#[from] sessions::Error),

    #[error("Connection error: {0}")]
    WebSocket(String),

    #[error("GraphQL error: {0:?}")]
    Graphql(Vec<String>),

    #[error("File read error: {0}")]
    FileRead(gloo::file::FileReadError),

    #[error("Local image already uploaded")]
    Reupload,

    #[error("Could not cast to the specified html element")]
    InvalidCast,

    #[error("Error executing javascript code")]
    JsError,

    #[error("Encountered an error while drawing on a canvas")]
    DrawError,

    #[error("Cannot select multiple files as image")]
    MultipleFiles,

    #[error("Quiz must have at least one round")]
    DeleteOnlyRound,

    #[error("Rounds are not linked to any quiz")]
    UnlinkedRounds,

    #[error("Cannot delete a quiz that is still being created")]
    DeleteUncommittedQuiz,

    #[error("No quiz with given id")]
    QuizNotFound,

    #[error("Must be authenticated to perform this action")]
    NotAuthenticated,
}

// pub type Result<T> = std::result::Result<T, Error>;

#[derive(serde::Deserialize, Clone, Debug, PartialEq)]
pub struct User {
    pub nickname: String,
    pub name: String,
    pub picture: String,
    pub updated_at: String,
    pub email: String,
    pub email_verified: bool,
    pub sub: String,
    pub token: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Auth {
    Loading,
    Anonymous,
    User(User),
}

impl Display for Auth {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::User(user) => f.write_fmt(format_args!("User({})", &user.name)),
            Self::Anonymous => f.write_str("Anonymous"),
            Self::Loading => f.write_str("Loading"),
        }
    }
}
