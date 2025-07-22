use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct LogAnalysis {
    pub counts: HashMap<String, usize>,         // Nombre de logs par LEVEL
    pub first_error_ts: Option<u64>,            // Timestamp du premier log ERROR
    pub min_max_ts: Option<(u64, u64)>,         // (min, max) timestamps vus
    pub warn_messages: Vec<String>,             // Messages des logs WARN
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    MalformedLine(String),
    InvalidTimestamp(String),
}

// C'est la fonction que tu dois implémenter.
pub fn analyze_logs(logs: &str) -> Result<LogAnalysis, ParseError> {
    // Ton implémentation ici.
    // Indice : l'accumulateur de ton `try_fold` sera `LogAnalysis`.
    // Tu devras probablement créer une fonction `LogAnalysis::new()` ou utiliser `Default`.
    logs.lines().try_fold(LogAnalysis::default(), |mut acc_log, line| {
         let mut split = line.split(':');
         let entry = split.next().ok_or(ParseError::MalformedLine(line.to_string() ))?;
         let time_stamp = split.next().ok_or(ParseError::MalformedLine(line.to_string() ))?;
         let message = split.next().ok_or(ParseError::MalformedLine(line.to_string() ))?;

         if let Some(_next) = split.next() {
            return Err(ParseError::MalformedLine(line.to_string()));
         }

         let parsed_time_stamp = time_stamp.parse::<u64>().map_err(|e| ParseError::InvalidTimestamp(e.to_string()))?;

         match entry {
            "INFO" => {
                *acc_log.counts.entry("INFO".to_string()).or_insert(0) += 1;
            },
            "WARN" => {
                *acc_log.counts.entry("WARN".to_string()).or_insert(0) += 1;
                acc_log.warn_messages.push(message.to_string());

            },
            "ERROR"=> {
                *acc_log.counts.entry("ERROR".to_string()).or_insert(0) += 1;
                if acc_log.first_error_ts.is_none() {
                    acc_log.first_error_ts = Some(parsed_time_stamp);
                }
            },
            _ => {
                return Err(ParseError::MalformedLine(line.to_string()));
            }
        }


        if let Some((min, max)) = acc_log.min_max_ts {
            acc_log.min_max_ts = Some((min.min(parsed_time_stamp), max.max(parsed_time_stamp)));
        } else {
            acc_log.min_max_ts = Some((parsed_time_stamp, parsed_time_stamp));
        }





        Ok(acc_log)
    } )
}

#[cfg(test)]
mod tests {
    use super::*;

    const LOG_DATA: &str = r#"INFO:1686770400:User logged in
WARN:1686770401:Password nearing expiration
INFO:1686770402:Data processed
ERROR:1686770403:Failed to connect to database
INFO:1686770404:User logged out
WARN:1686770405:Disk space low"#;

    const LOG_DATA_WITH_ERROR: &str = r#"INFO:1686770400:User logged in
WARN:1686770401:Password nearing expiration
THIS_IS_A_BAD_LINE
ERROR:1686770403:Failed to connect to database"#;
    
    #[test]
    fn test_successful_analysis() {
        let analysis = analyze_logs(LOG_DATA).unwrap();
        
        let mut expected_counts = HashMap::new();
        expected_counts.insert("INFO".to_string(), 3);
        expected_counts.insert("WARN".to_string(), 2);
        expected_counts.insert("ERROR".to_string(), 1);

        assert_eq!(analysis.counts, expected_counts);
        assert_eq!(analysis.first_error_ts, Some(1686770403));
        assert_eq!(analysis.min_max_ts, Some((1686770400, 1686770405)));
        assert_eq!(analysis.warn_messages, vec!["Password nearing expiration".to_string(), "Disk space low".to_string()]);
    }

    #[test]
    fn test_parsing_error() {
        let result = analyze_logs(LOG_DATA_WITH_ERROR);
        assert_eq!(result, Err(ParseError::MalformedLine("THIS_IS_A_BAD_LINE".to_string())));
    }

    #[test]
    fn test_empty_input() {
        let analysis = analyze_logs("").unwrap();
        assert_eq!(analysis.counts, HashMap::new());
        assert_eq!(analysis.first_error_ts, None);
        assert_eq!(analysis.min_max_ts, None);
        assert!(analysis.warn_messages.is_empty());
    }

    #[test]
    fn test_first_error_timestamp() {
        let logs = "ERROR:100:First error\nERROR:200:Second error";
        let analysis = analyze_logs(logs).unwrap();
        assert_eq!(analysis.first_error_ts, Some(100));
    }
}