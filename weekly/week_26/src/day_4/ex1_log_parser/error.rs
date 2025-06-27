use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    TimeStamp(#[from] chrono::ParseError),
    
    #[error("Generique")]
    Generique,

    #[error(transparent)]
    Clap(#[from] clap::Error),

    #[error("Critical condition meet")]
    CriticalConditionMeet,

    #[error("Unvalid input: {input}")]
    UnvalidInput { input: String },

    #[error("Unable to parsing line: {line}")]
    UnvalidLine { line: String },

    #[error("Parsing aborted: line number {number} is empty")]
    EmptyLineParse { number: usize },
}

pub type Result<T> = std::result::Result<T, AppError>;
