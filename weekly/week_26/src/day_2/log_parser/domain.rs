use std::collections::HashMap;

use crate::error::Domain;

use chrono::{DateTime, TimeDelta, Utc};

//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==DATASTRUCTURE

pub struct LogEntryDto {
    pub raw: String,
}

pub struct ParsedLogEntry {
    pub time_stamp: TimeStamp,
    pub level: LogLevel,
    pub user_id: User,
    pub operation: Operation,
    pub data: Option<String>,
}

impl ParsedLogEntry {
    pub fn default() -> Result<Self, Domain> {
        Ok(Self {
            time_stamp: TimeStamp::new("1970-01-01T00:00:00Z")?,
            level: LogLevel::Default,
            user_id: User::default(),
            operation: Operation::Default,
            data: None,
        })
    }
}


//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==USER
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub struct User {
    pub id: String,
}

impl User {
    pub fn new(input: &str) -> Self {
        let id = input;
        Self { id: id.to_string()}
    }
}
//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==TIMESTAMP
#[derive(Debug, Clone)]
pub struct TimeStamp {
    pub time: DateTime<Utc>,
}

impl TimeStamp {
    pub fn new(raw_date: &str) -> Result<Self, Domain> {
        let time = DateTime::parse_from_rfc3339(&raw_date)?.with_timezone(&Utc);

        Ok(Self { time })
    }
}


//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==LOGLEVEL
#[derive(Debug,  Clone, PartialEq, Eq, PartialOrd, Hash)]
pub enum LogLevel {
    Default,
    Info,
    Warn,
    Error,
}

//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==OPERATIONS
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub enum Operation {
    Default,
    Login,
    Logout,
    CreateDocument,
    DeleteDocument,
    ReadDocument,
}


//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==STATS

#[derive(Debug, Default)]
pub struct AnalysisReport {
    pub total_lines: u32,
    pub parsed_lines: u32,
    pub corrupted_lines: u32,
    pub log_count: u32,
    pub last_log: HashMap<User,(Operation,TimeStamp)>,
    pub level_distribution: HashMap<LogLevel, u32>,
    pub user_activity: HashMap<User, u32>, //min max 
    pub log_duration: HashMap<User, TimeDelta>,
    pub operation_nbr: u32,
}




