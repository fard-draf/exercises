use chrono::TimeDelta;

use crate::{
    analyzer,
    domain::{AnalysisReport, LogLevel, ParsedLog, TimeStamp},
    error::*,
    parser::log_parser,
};

use std::collections::HashMap;

pub fn analyzer_report(input: &str) -> Result<AnalysisReport> {
    if input.is_empty() {
        return Err(AppError::UnvalidInput {
            input: input.to_string(),
        });
    }

    let init_analysis = AnalysisReport::default();

    let mut res = input
        .lines()
        .enumerate()
        .fold(init_analysis, |mut acc, (line_index, line)| {
            acc.proceed_lines += 1;

            if let Ok(parsed_line) = log_parser(line, line_index) {
                acc.parsed_lines += 1;
                *acc.log_level_counter
                    .entry(parsed_line.level.clone())
                    .and_modify(|e| *e += 1)
                    .or_insert(1);

                match &parsed_line.level {
                    LogLevel::Error => {
                        acc.error_log.push(parsed_line.time_stamp.clone());

                        if let Some(key) = acc.log_level_counter.get(&LogLevel::Error) {
                            acc.log_error_timeline.push((parsed_line.time_stamp, *key));
                        }
                    }
                    _ => {}
                }
            } else {
                acc.parse_error_counter += 1;
            }

            acc
        });

    if !res.error_log.is_empty() {
        let error_log_window = res.error_log[..res.error_log.len() - 1].windows(3);
        let burst = error_log_window
            .filter_map(|window| {
                let first_log = &window[0];
                let last_log = &window[2];

                let delta = last_log.clone() - first_log.clone();
                if delta <= TimeDelta::seconds(5) {
                    Some(first_log.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<TimeStamp>>();

        let total_error_over_time: Vec<(TimeStamp, usize)> = {
            burst
                .iter()
                .scan(0, |cumulative_count, error_log| {
                    *cumulative_count += 1;
                    Some((error_log.clone(), *cumulative_count))
                })
                .collect()
        };
        res.burst_error = total_error_over_time;
    }

    Ok(res)
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn test_analyzer_without_logerror() {
        let input = "2025-06-27T10:00:01Z [INFO] Application starting up.
2025_06-27T10:00:05Z [INFO] Database connection established.
2025-06-27T10:01:10Z [WARNING] Configuration value 'timeout' is deprecated.";

        let mut log_level_hash = HashMap::new();
        *log_level_hash.entry(LogLevel::Info).or_insert(0) += 1;
        *log_level_hash.entry(LogLevel::Warning).or_insert(0) += 1;

        let analyzer = AnalysisReport {
            proceed_lines: 3,
            parsed_lines: 2,
            parse_error_counter: 1,
            log_level_counter: log_level_hash,
            burst_error: vec![],
            error_log: Vec::<TimeStamp>::new(),
            log_error_timeline: Vec::<(TimeStamp, u32)>::new(),
        };

        let result = analyzer_report(input).unwrap();

        assert_eq!(result, analyzer);
    }

    #[test]
    fn test_analyzer_with_logerror() {
        let input = "2025-06-27T10:00:00Z [ERROR] Application starting up.
2025-06-27T10:00:05Z [INFO] Database connection established.
2025-06-27T10:01:10Z [WARNING] Configuration value 'timeout' is deprecated.
2025-06-27T10:02:02Z [ERROR] Circuit breaker opened for upstream service.
2025-06-27T10:02:03Z [ERROR] Circuit breaker opened for upstream service.
2025_06-27T10:02:04Z [ERROR] This line had an error (DateTime format).
2025-06-27T10:02:07Z [ERROR] Circuit breaker opened for upstream service.
2025-06-27T10:02:08Z [ERROR] Critical error, system will shutdown in 5 seconds..";

        let mut log_level_hash = HashMap::new();
        *log_level_hash.entry(LogLevel::Error).or_insert(0) += 5;
        *log_level_hash.entry(LogLevel::Info).or_insert(0) += 1;
        *log_level_hash.entry(LogLevel::Warning).or_insert(0) += 1;

        let time_stamp1 = TimeStamp::from_str("2025-06-27T10:00:00Z").unwrap();
        let time_stamp2 = TimeStamp::from_str("2025-06-27T10:02:02Z").unwrap();
        let time_stamp3 = TimeStamp::from_str("2025-06-27T10:02:03Z").unwrap();
        // let time_stamp4 = TimeStamp::from_str("2025-06-27T10:02:04Z").unwrap();
        let time_stamp5 = TimeStamp::from_str("2025-06-27T10:02:07Z").unwrap();
        let time_stamp6 = TimeStamp::from_str("2025-06-27T10:02:08Z").unwrap();

        let mut vec_timestamp = vec![time_stamp1.clone()];
        vec_timestamp.push(time_stamp2.clone());
        vec_timestamp.push(time_stamp3.clone());
        // vec_timestamp.push(time_stamp4.clone());
        vec_timestamp.push(time_stamp5.clone());
        vec_timestamp.push(time_stamp6.clone());

        let mut vec_timeline = vec![(time_stamp1.clone(), 1)];
        vec_timeline.push((time_stamp2, 2));
        vec_timeline.push((time_stamp3, 3));
        // vec_timeline.push((time_stamp4, 4));
        vec_timeline.push((time_stamp5, 4));
        vec_timeline.push((time_stamp6, 5));

        let analyzer = AnalysisReport {
            proceed_lines: 8,
            parsed_lines: 7,
            parse_error_counter: 1,
            log_level_counter: log_level_hash,
            burst_error: vec![((TimeStamp::from_str("2025-06-27T10:02:02Z").unwrap(), 1))],
            error_log: vec_timestamp,
            log_error_timeline: vec_timeline,
        };

        let result = analyzer_report(input).unwrap();

        assert_eq!(result, analyzer);
    }
}
