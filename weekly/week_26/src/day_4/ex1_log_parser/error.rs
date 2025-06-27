use thiserror::Error;


#[derive(Debug, Error)]
pub enum AppError {
    // #[error()]
}


pub type Result<T> = std::result::Result<T, AppError>;