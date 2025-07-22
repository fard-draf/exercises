use crate::{
    domain::{LogLevel, Operation, ParsedLogEntry, TimeStamp, User},
    error::*,
};

pub fn parse_log(line: &str) -> Result<ParsedLogEntry> {
    let mut acc = ParsedLogEntry::default()?;

    let parts: Vec<&str> = line.splitn(5, ';').map(str::trim).collect();

    if !(4..=5).contains(&parts.len()) {
        return Ok::<ParsedLogEntry, AppError>(acc);
    }

    acc.time_stamp = TimeStamp::new(parts[0])?;
    acc.level = match parts[1].to_lowercase().as_str() {
        "info" => LogLevel::Info,
        "warn" => LogLevel::Warn,
        "error" => LogLevel::Error,
        _ => LogLevel::Default,
    };
    acc.user_id = User::new(parts[2]);

    acc.operation = match parts[3].to_lowercase().as_str() {
        "login" => Operation::Login,
        "logout" => Operation::Logout,
        "create_document" => Operation::CreateDocument,
        "delete_document" => Operation::DeleteDocument,
        "read_document" => Operation::ReadDocument,
        _ => Operation::Default,
    };
    acc.data = parts.get(4).map(|s| s.to_string());

    Ok(acc)
}
