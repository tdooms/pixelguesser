use wasm_bindgen::JsValue;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Api error {0}")]
    Api(#[from] api::Error),

    #[error("Connection error: {0}")]
    WebSocket(String),

    #[error("Could not cast to the specified html element")]
    InvalidCast,

    #[error("Pixelation component could not load image")]
    ImageCouldNotLoad,

    #[error("Error executing javascript code")]
    JsError,

    #[error("Encountered an error while drawing on a canvas")]
    DrawError,

    #[error("Cannot select multiple files as image")]
    MultipleFiles,

    #[error("Error while initializing authentication: {0}")]
    AuthInit(String),

    #[error("Could not deserialize user: {0}")]
    UserSerde(#[from] serde_wasm_bindgen::Error),

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
