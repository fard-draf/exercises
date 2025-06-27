use chrono::{DateTime, TimeDelta, Utc};

#[derive(Debug, Default)]
pub struct ParsedLog {
    pub time_stamp: DateTime<Utc>,
    pub level: LogLevel,
    pub message: String,
}


#[derive(Debug)]
pub enum LogLevel {
    Default,
    Info,
    Warning,
    Error,
}

impl Default for  LogLevel {
    fn default() -> LogLevel {
        LogLevel::Default
    }
}

