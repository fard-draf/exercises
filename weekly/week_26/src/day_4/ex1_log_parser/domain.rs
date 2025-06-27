use crate::error::*;

use std::{collections::HashMap, ops::Sub, str::FromStr};
use chrono::{DateTime, TimeDelta, Utc};



#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParsedLog {
    pub time_stamp: TimeStamp,
    pub level: LogLevel,
    pub message: String,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimeStamp(DateTime<Utc>);

impl TimeStamp {
    pub fn from_str(data: &str) -> Result<Self> {
        let date_time = DateTime::parse_from_rfc3339(&data)?.with_timezone(&Utc);
        Ok(Self(date_time))
    }
}

impl Sub for TimeStamp {
    type Output = TimeDelta;

    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LogLevel {
    Default,
    Info,
    Warning,
    Error,
}

impl Default for LogLevel {
    fn default() -> LogLevel {
        LogLevel::Default
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct AnalysisReport {
    pub proceed_lines: u32, // total lines
    pub parsed_lines: u32,  // lines parsed
    pub parse_error_counter: u32, // lines with parsing err
    pub log_level_counter: HashMap<LogLevel, u32>,
    pub burst_error: Vec<(TimeStamp, u32)>, 
    pub log_error_timestamp: Vec<TimeStamp>,
    pub log_error_timeline: Vec<(TimeStamp, u32)>,

}


//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==//==TEST
#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn test_good_timestamp() {
        let raw_time_stamp = "1970-01-01T00:00:00Z";

        assert!(TimeStamp::from_str(raw_time_stamp).is_ok());
    }

    #[test]
    fn test_bad_timestamp() {
        let raw1 = "19700101T 00:00:00Z";
        let raw2 = "1970-01-01H00:00:00Z";
        let raw3 = "1970-01-01T00:00:00O";
        let raw4 = "1970_01-01T00:00:00Z";

        assert!(TimeStamp::from_str(raw1).is_err());
        assert!(TimeStamp::from_str(raw2).is_err());
        assert!(TimeStamp::from_str(raw3).is_err());
        assert!(TimeStamp::from_str(raw4).is_err());
    }

    #[test]
    fn test_error_kind() {
        let raw1 = "19700101T 00:00:00Z";
        let parsed_time = TimeStamp::from_str(raw1);
        assert!(parsed_time.is_err());

        match parsed_time.unwrap_err() {
            AppError::TimeStamp(e) => println!("Erreur correcte: {e}"),
            _ => panic!("Mauvais type d'erreur"),
        }
    }
}
