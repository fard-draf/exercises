use thiserror::Error;

#[derive(Debug, Error)]
pub enum PackError {
    #[error("Buffer too small, expected {expected}, got {actual}")]
    BufferTooSmall { expected: usize, actual: usize },

    #[error("Invalid account state: {0}")]
    InvalidState(u8),

    #[error("Invalid data alignment")]
    InvalidAlignment,

    #[error("Account not initialized")]
    NotInitialized,

    #[error("Unvalid length")]
    UnvalidLength,
}
