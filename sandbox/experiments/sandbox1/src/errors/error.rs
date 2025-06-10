use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Generic error 1")]
    GeneriqueError1,

    #[error("Error: {0}")]
    MessageForwardedByString(String),
}
