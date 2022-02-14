use yew_router::prelude::*;

pub const HOST_ROUND_START_TIME: u32 = 5_000;
pub const HOST_AFTER_REVEALED_TIME: u32 = 1_000;
pub const PIXELATE_REFRESH_TIME: u32 = 100;
pub const PIXELATE_START_PIXELS: f64 = 4.0;
pub const PIXELATE_PLAY_SPEED: f64 = 1.002;
pub const PIXELATE_REVEAL_SPEED: f64 = 1.07;
pub const CREATE_LONG_SAVE_TIME: u32 = 10_000;
pub const CREATE_SHORT_SAVE_TIME: u32 = 1_000;

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
    #[error("Api error {0}")]
    Api(#[from] api::Error),

    #[error("Connection error: {0}")]
    WebSocket(String),

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
