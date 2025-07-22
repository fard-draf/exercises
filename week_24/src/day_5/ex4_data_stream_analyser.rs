use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct StreamAnalysis {
    total_readings: usize,
    anomaly_count: usize,
    sensor_averages: HashMap<String, f64>,
    time_span: (u64, u64),
}

fn analyze_stream(data: &[&str]) -> StreamAnalysis {
    // Accumulator pour fold unique
    #[derive(Default)]
    struct Accumulator {
        total_readings: usize,
        anomaly_count: usize,
        sensor_sums: HashMap<String, (f64, usize)>, // (sum, count)
        first_timestamp: Option<u64>,
        last_timestamp: u64,
    }

    if data.is_empty() {
        return StreamAnalysis {
            total_readings: 0,
            anomaly_count: 0,
            sensor_averages: HashMap::new(),
            time_span: (0, 0),
        };
    }

    // TODO: Implémenter avec fold unique
    let values = data.iter().fold(Accumulator::default(), |mut acc, value| {
        let parsed_data = parse_data(value);

        //TIME
        if let Some(timestamp) = parsed_data.0 {
            if acc.first_timestamp.is_none() {
                acc.first_timestamp = Some(timestamp);
            }
            acc.last_timestamp = timestamp;
        }
        //SENSOR
        if let Some((data_type, data)) = parsed_data.1 {
            if parsed_data.2 == Some("ok") {
                acc.sensor_sums
                    .entry(data_type.to_string())
                    .and_modify(|e| {
                        e.0 += data;
                        e.1 += 1;
                    })
                    .or_insert((data, 1));
            }
        }

        //ANOMALY
        if let Some(anomaly) = parsed_data.2 {
            match anomaly {
                "ok" => {
                    acc.total_readings += 1;
                }
                "anomaly" => {
                    acc.total_readings += 1;
                    acc.anomaly_count += 1;
                }
                _ => {
                    acc.anomaly_count += 1;
                }
            };
        }

        acc
    });

    let result = values
        .sensor_sums
        .into_iter()
        .map(|(string, values)| (string, values.0 / values.1 as f64))
        .collect::<HashMap<String, f64>>();

    println!("result :{:#?}", result);

    StreamAnalysis {
        total_readings: values.total_readings,
        anomaly_count: values.anomaly_count,
        sensor_averages: result,
        time_span: (values.first_timestamp.unwrap(), values.last_timestamp),
    }
}

fn parse_data(value: &str) -> (Option<u64>, Option<(&str, f64)>, Option<&str>) {
    let mut splitted = value.split(':');
    let time = splitted.next();
    let sensor_type = splitted.next();
    let sensor_value = splitted.next();
    let anomaly = splitted.next();

    let time = if let Some(time) = time {
        time.parse::<u64>().ok()
    } else {
        None
    };

    let sensor_value = if let Some(value) = sensor_value {
        value.parse::<f64>().ok()
    } else {
        None
    };

    let sensor = sensor_type.zip(sensor_value);
    (time, sensor, anomaly)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stream_analysis() {
        let data = &[
            "1623456789:temp_01:23.5:ok",
            "1623456790:temp_01:24.1:ok",
            "1623456791:temp_01:45.8:anomaly",
            "1623456792:humidity_02:65.2:ok",
            "1623456793:temp_01:23.9:ok",
        ];

        let result = analyze_stream(data);

        assert_eq!(result.total_readings, 5);
        assert_eq!(result.anomaly_count, 1);
        assert_eq!(result.time_span, (1623456789, 1623456793));

        // temp_01: (23.5 + 24.1 + 23.9) / 3 = 23.83
        assert!((result.sensor_averages.get("temp_01").unwrap() - 23.833).abs() < 0.01);
        assert!((result.sensor_averages.get("humidity_02").unwrap() - 65.2).abs() < 0.01);
    }

    #[test]
    fn test_malformed_data() {
        let data = &[
            "invalid_line",
            "1623456789:temp_01:23.5:ok",
            "incomplete:data",
        ];

        let result = analyze_stream(data);
        assert_eq!(result.total_readings, 1); // seule ligne valide
    }

    #[test]
    fn test_empty_stream() {
        let result = analyze_stream(&[]);
        assert_eq!(result.total_readings, 0);
        assert_eq!(result.anomaly_count, 0);
        assert_eq!(result.time_span, (0, 0));
    }
}
