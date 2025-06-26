use chrono::{DateTime, FixedOffset, ParseResult};
use thiserror::Error;

#[derive(Error, Debug)]

pub enum AppError {
    #[error(transparent)]
    Domain(#[from] Domain),

    #[error("Parsing Error: {line_content}")]
    ParsingError { line_content: String },

    #[error(transparent)]
    CommandLine(#[from] CommandLine),

    #[error(transparent)]
    IO(#[from] std::io::Error),
}

#[derive(Debug, Error)]
pub enum Domain {
    #[error(transparent)]
    Time(#[from] chrono::ParseError),
}

#[derive(Debug, Error)]
pub enum CommandLine {
    #[error("Missing args")]
    MissingArgs,
}

pub type Result<T> = std::result::Result<T, AppError>;
