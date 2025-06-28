#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub struct WeatherReading {
    pub hour: u8,
    pub temperature: f64,
    pub precipitation_mm: f64,
}

#[derive(Debug, PartialEq, Default)]
pub struct DailyWeatherReport {
    pub avg_temp: f64,
    pub total_precipitation: f64,
    pub cumulative_precipitation: Vec<f64>,
    pub max_temp_swing: f64,
}

pub fn analyze_weather_data(readings: impl Iterator<Item = WeatherReading>) -> DailyWeatherReport {
    let initial_data = (DailyWeatherReport::default(), 0, 0.0);

    let readings = readings.collect::<Vec<WeatherReading>>();

    if readings.is_empty() {
        return DailyWeatherReport::default();
    }

    let (mut report, count, _) = readings.iter().enumerate().fold(
        initial_data,
        |(mut report, mut counter, mut prev_temp), (index, data)| {
            counter += 1;
            report.avg_temp += data.temperature;

            report.total_precipitation += data.precipitation_mm;
            report
                .cumulative_precipitation
                .push(report.total_precipitation);

            if index != 0 {
                let delta = (data.temperature - prev_temp).abs();
                if delta > report.max_temp_swing {
                    report.max_temp_swing = delta;
                }
            }

            prev_temp = data.temperature;

            (report, counter, prev_temp)
        },
    );

    report.avg_temp = report.avg_temp / (count as f64);

    report
}

#[cfg(test)]
mod tests {
    use super::*;

    fn f64_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn test_nominal_case() {
        let readings = vec![
            WeatherReading {
                hour: 0,
                temperature: 10.0,
                precipitation_mm: 0.0,
            },
            WeatherReading {
                hour: 1,
                temperature: 12.0,
                precipitation_mm: 0.0,
            },
            WeatherReading {
                hour: 2,
                temperature: 11.0,
                precipitation_mm: 5.0,
            },
            WeatherReading {
                hour: 3,
                temperature: 13.0,
                precipitation_mm: 2.0,
            },
        ];

        let report = analyze_weather_data(readings.into_iter());

        let expected_avg_temp = (10.0 + 12.0 + 11.0 + 13.0) / 4.0; // 11.5
        let expected_total_precip = 7.0;
        let expected_cumulative_precip = vec![0.0, 0.0, 5.0, 7.0];
        let expected_max_swing = 2.0;

        assert!(
            f64_eq(report.avg_temp, expected_avg_temp),
            "Temp. moyenne incorrecte: attendu {}, obtenu {}",
            expected_avg_temp,
            report.avg_temp
        );
        assert!(
            f64_eq(report.total_precipitation, expected_total_precip),
            "Précip. totale incorrecte"
        );
        assert_eq!(
            report.cumulative_precipitation, expected_cumulative_precip,
            "Précip. cumulées incorrectes"
        );
        assert!(
            f64_eq(report.max_temp_swing, expected_max_swing),
            "Variation max de temp. incorrecte"
        );
    }

    #[test]
    fn test_empty_case() {
        let readings: Vec<WeatherReading> = vec![];
        let report = analyze_weather_data(readings.into_iter());
        assert_eq!(report, DailyWeatherReport::default());
    }
}
