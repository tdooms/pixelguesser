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

    #[error("Could not find the canvas to pixelate")]
    PixelationCanvasError,

    #[error("Encountered an error while drawing the pixelated canvas")]
    PixelationDrawError,

    #[error("Encountered an error with javascript attributes of pixelation canvas")]
    PixelationJsError,

    #[error("Encountered an error when querying the window, probably due to web workers")]
    WindowError,
}

#[derive(thiserror::Error, Debug)]
pub enum Success {}

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
    #[error("{0}")]
    Success(Success),
}
