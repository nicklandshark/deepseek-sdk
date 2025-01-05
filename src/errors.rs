use thiserror::Error as ThisError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Building failed: {0}")]
    BuildFailed(&'static str),

    #[error("Request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("Serde failed: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
}
