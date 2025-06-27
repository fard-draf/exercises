use crate::domain::{LogLevel, ParsedLog, TimeStamp};
use crate::error::{AppError, Result};

pub fn log_parser(line: &str, line_index: usize) -> Result<ParsedLog> {
    if line.is_empty() {
        return Err(AppError::EmptyLineParse { number: line_index });
    }

    let parts = line.splitn(3, ' ').collect::<Vec<&str>>();

    parts.iter().try_fold(ParsedLog::default(), |mut acc, _| {
        acc.time_stamp = TimeStamp::from_str(parts[0].trim())?;
        acc.level = match parts[1].trim().to_lowercase().as_ref() {
            "[info]" => LogLevel::Info,
            "[warning]" => LogLevel::Warning,
            "[error]" => LogLevel::Error,
            _ => LogLevel::Default,
        };
        acc.message = parts[2].trim().to_string();

        Ok(acc)
    })
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn fn_parsing_line() {
        let line = "2025-06-27T10:00:01Z [INFO] Application starting up.";
        let result = log_parser(line, 1).unwrap();

        let test_line = ParsedLog {
            time_stamp: TimeStamp::from_str("2025-06-27T10:00:01Z").unwrap(),
            level: LogLevel::Info,
            message: "Application starting up.".to_string(),
        };

        assert_eq!(result, test_line);
    }
    #[test]
    fn test_parsing_lines() {
        let lines = "2025-06-27T10:00:01Z [INFO] Application starting up.
2025-06-27T10:00:05Z [INFO] Database connection established.
2025-06-27T10:01:10Z [WARNING] Configuration value 'timeout' is deprecated.
2025-06-27T10:02:00Z [ERROR] Failed to process transaction 1A4B: upstream service unavailable.
2025-06-27T10:02:01Z [ERROR] Failed to process transaction C82F: upstream service unavailable.
2025-06-27T10:02:03Z [INFO] Retrying connection to upstream service.
2025-06-27T10:02:04Z [ERROR] Failed to process transaction 9F0E: upstream service unavailable.
2025-06-27T10:02:05Z [ERROR] Retrying failed, escalating issue.
2025-06-27T10:02:06Z [ERROR] Circuit breaker opened for upstream service.
2025-06-27T10:05:00Z [INFO] System recovered.";

        let result = lines
            .lines()
            .enumerate()
            .map(|(index, line)| log_parser(line, index).unwrap())
            .collect::<Vec<ParsedLog>>();

        let last = result.last().unwrap();
        let test = ParsedLog {
            time_stamp: TimeStamp::from_str("2025-06-27T10:05:00Z").unwrap(),
            level: LogLevel::Info,
            message: "System recovered.".to_string(),
        };
        assert_eq!(*last, test);
    }

    #[test]
    fn test_empty_line() {
        let line = "";

        assert!(log_parser(line, 1).is_err());
    }

    #[test]
    fn fn_err_parse() {
        let line = "2025_06-27T10:02:04Z [ERROR] Failed to process transaction 9F0E: upstream service unavailable.";
        let result = log_parser(line, 1);

        println!("{:?}", result);
        assert!(result.is_err());

        match result.unwrap_err() {
            AppError::TimeStamp(e) => println!("Le test a capture l'erreur suivante: {e}"),
            _ => panic!("Mauvais type d'erreur"),
        }
    }
}
