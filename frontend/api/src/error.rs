use cynic::DecodeError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Request error: {0}")]
    Request(#[from] reqwasm::Error),

    #[error("Decode error {0}")]
    Decode(#[from] DecodeError),

    #[error("Encode error {0}")]
    Encode(#[from] cynic::serde_json::Error),
}
