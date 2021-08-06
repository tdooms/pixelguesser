#[derive(thiserror::Error, Debug)]
pub enum Info {
    #[error("Lost connection, trying to restore")]
    WsDisconnect,

    #[error("Your current session was stopped by the host")]
    SessionStopped,

    #[error("Your current session is not valid (anymore)")]
    SessionInvalid,
}
