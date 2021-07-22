#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("The websocket connection encountered an error: {0}")]
    WsError(#[from] anyhow::Error),

    #[error("Could not (de)serialize response/request from websocket: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Server could not create a new session")]
    SessionCreationFailed,

    #[error("Player does not exist")]
    PlayerDoesNotExist,

    #[error("The server encountered an error")]
    InternalServerError,

    #[error("The server received a faulty request")]
    FaultyRequest,

    #[error("Could not cast to the specified html element")]
    InvalidCast,

    #[error("Error executing javascript code")]
    JsError,

    #[error("Encountered an error while drawing on a canvas")]
    DrawError,
}

#[derive(thiserror::Error, Debug)]
pub enum Warning {
    #[error("Lost connection, trying to restore")]
    WsDisconnect,

    #[error("Your current session was stopped by the host")]
    SessionStopped,

    #[error("Your current session is not valid (anymore)")]
    SessionInvalid,
}

#[derive(thiserror::Error, Debug)]
pub enum Notification {
    #[error("{0}")]
    Error(Error),
    #[error("{0}")]
    Warning(Warning),
}
