#[derive(Debug, Clone)]
pub struct Reading {
    pub temperature: f32,
    pub timestamp: u32,
    pub valid: bool,
}

#[derive(Debug, PartialEq)]
pub struct SensorAnalysis {
    pub average_temp: Option<f32>,
    pub min_max: Option<(f32, f32)>,
    pub invalid_count: usize,
    pub anomalies: Vec<u32>,
    pub stable_periods: Vec<(u32, u32)>,
}

// Structure d'accumulateur suggérée (tu peux adapter)
#[derive(Debug, Default)]
struct Accumulator {
    // Pour les stats
    sum: f32,
    count: usize,
    min_max: Option<(f32, f32)>,
    invalid_count: usize,

    // Pour les anomalies
    last_valid_temp: Option<f32>,
    anomalies: Vec<u32>,

    // Pour les périodes stables
    stable_start: Option<u32>,
    last_stable_period: Option<u32>,
    stable_temps: Vec<f32>,
    stable_periods: Vec<(u32, u32)>,
}

pub fn analyze_sensor_data(readings: &[Reading]) -> SensorAnalysis {
    const TEMP_MAX: f32 = 100f32;
    const TEMP_MIN: f32 = -40f32;
    let data = readings
        .iter()
        .fold(Accumulator::default(), |mut acc, data| {
            if data.valid {
                match data {
                    Reading {
                        temperature,
                        timestamp,
                        valid,
                    } => {
                        acc.sum = {
                            if *temperature >= TEMP_MIN && *temperature <= TEMP_MAX {
                                let value = acc.sum + data.temperature;
                                println!("acc sum {}, temp {}", acc.sum, data.temperature);
                                acc.count += 1;
                                println!("acc count {}", acc.count);
                                value
                            } else {
                                acc.sum
                            }
                        };
                        acc.min_max = {
                            if let Some(value) = acc.min_max {
                                let min = value.0.min(data.temperature);
                                let max = value.1.max(data.temperature);
                                Some((min, max))
                            } else {
                                Some((data.temperature, data.temperature))
                            }
                        };

                        acc.invalid_count = {
                            if let Some(value) = acc.min_max {
                                if acc.min_max.unwrap().1 - acc.min_max.unwrap().0 > 10f32
                                    || data.temperature > TEMP_MAX
                                    || data.temperature < TEMP_MIN
                                {
                                    acc.invalid_count + 1usize
                                } else {
                                    acc.last_valid_temp = Some(*temperature);
                                    acc.invalid_count
                                }
                            } else {
                                acc.invalid_count
                            }
                        };
                        acc.anomalies = {
                            if let Some(value) = acc.min_max {
                                let variation = acc.min_max.unwrap().1 - acc.min_max.unwrap().0;
                                if variation > 1.0 {
                                    // acc.anomalies.pop();
                                    acc.anomalies.truncate(1);
                                    acc.anomalies.push(data.timestamp);
                                    acc.anomalies
                                } else {
                                    acc.anomalies
                                }
                            } else {
                                acc.anomalies
                            }
                        };
                        acc.stable_start = {
                            if let Some(value) = acc.stable_start {
                                Some(value)
                            } else {
                                Some(data.timestamp)
                            }
                        };
                        acc.last_stable_period = {
                            if let Some(value) = acc.last_stable_period {
                                Some(value)
                            } else {
                                Some(data.timestamp)
                            }
                        };
                        acc.stable_temps = {
                            acc.stable_temps.push(data.temperature);
                            acc.stable_temps
                        };
                        acc.stable_periods = {
                            let mut variation = acc.min_max.unwrap().1 - acc.min_max.unwrap().0;
                            println!("variation {}", variation);

                            if variation > 1.0 {}

                            acc.stable_periods

                            // if let Some(value) = acc.min_max {
                            //     let variation = acc.min_max.unwrap().1 - acc.min_max.unwrap().0;

                            //     if variation > 1.0 {
                            //         if let Some(value) = acc.stable_start {
                            //             acc.stable_periods.push((value, data.timestamp));
                            //         } else {
                            //             acc.stable_periods.clone();
                            //         }
                            //         println!("it here");
                            //         acc.stable_periods
                            //     } else {
                            //         acc.stable_periods
                            //     }
                            // } else {
                            //     acc.stable_periods
                            // }
                        }
                    }
                }
            };
            &acc;
            if !data.valid {
                match data {
                    Reading {
                        temperature,
                        timestamp,
                        valid,
                    } => {
                        &acc.sum;
                        acc.count;
                        acc.min_max;
                        acc.invalid_count = acc.invalid_count + 1;
                        &acc.anomalies;
                        &acc.stable_periods;
                    }
                }
            };
            acc
        });

    println!("count {}", data.count);
    println!("temp sum {}", data.sum);
    let sensor = SensorAnalysis {
        average_temp: Some(data.sum / data.count as f32),
        min_max: data.min_max,
        invalid_count: data.invalid_count,
        anomalies: data.anomalies,
        stable_periods: data.stable_periods,
    };
    sensor
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_stats() {
        let readings = vec![
            Reading {
                temperature: 20.0,
                timestamp: 100,
                valid: true,
            },
            Reading {
                temperature: 21.0,
                timestamp: 200,
                valid: true,
            },
            Reading {
                temperature: 999.0,
                timestamp: 300,
                valid: false,
            },
            Reading {
                temperature: 22.0,
                timestamp: 400,
                valid: true,
            },
        ];

        let analysis = analyze_sensor_data(&readings);

        assert_eq!(analysis.average_temp, Some(21.0));
        assert_eq!(analysis.min_max, Some((20.0, 22.0)));
        assert_eq!(analysis.invalid_count, 1);
    }

    #[test]
    fn test_anomalies() {
        let readings = vec![
            Reading {
                temperature: 20.0,
                timestamp: 100,
                valid: true,
            },
            Reading {
                temperature: 20.5,
                timestamp: 200,
                valid: true,
            },
            Reading {
                temperature: 26.0,
                timestamp: 300,
                valid: true,
            }, // Anomalie!
            Reading {
                temperature: 26.5,
                timestamp: 400,
                valid: true,
            },
            Reading {
                temperature: 19.0,
                timestamp: 500,
                valid: true,
            }, // Anomalie!
        ];

        let analysis = analyze_sensor_data(&readings);

        assert_eq!(analysis.anomalies, vec![300, 500]);
    }

    #[test]
    fn test_stable_periods() {
        let readings = vec![
            // Période stable 1
            Reading {
                temperature: 20.0,
                timestamp: 100,
                valid: true,
            },
            Reading {
                temperature: 20.2,
                timestamp: 200,
                valid: true,
            },
            Reading {
                temperature: 20.1,
                timestamp: 300,
                valid: true,
            },
            Reading {
                temperature: 20.3,
                timestamp: 400,
                valid: true,
            },
            Reading {
                temperature: 20.2,
                timestamp: 500,
                valid: true,
            },
            Reading {
                temperature: 20.4,
                timestamp: 600,
                valid: true,
            },
            // Cassure
            Reading {
                temperature: 25.0,
                timestamp: 700,
                valid: true,
            },
            // Période stable 2
            Reading {
                temperature: 25.1,
                timestamp: 800,
                valid: true,
            },
            Reading {
                temperature: 25.0,
                timestamp: 900,
                valid: true,
            },
            Reading {
                temperature: 25.2,
                timestamp: 1000,
                valid: true,
            },
            Reading {
                temperature: 25.1,
                timestamp: 1100,
                valid: true,
            },
            Reading {
                temperature: 25.0,
                timestamp: 1200,
                valid: true,
            },
        ];

        let analysis = analyze_sensor_data(&readings);

        assert_eq!(analysis.stable_periods, vec![(100, 600), (800, 1200)]);
    }

    #[test]
    fn test_all_invalid() {
        let readings = vec![
            Reading {
                temperature: 20.0,
                timestamp: 100,
                valid: false,
            },
            Reading {
                temperature: 21.0,
                timestamp: 200,
                valid: false,
            },
        ];

        let analysis = analyze_sensor_data(&readings);

        assert_eq!(analysis.average_temp, None);
        assert_eq!(analysis.min_max, None);
        assert_eq!(analysis.invalid_count, 2);
        assert!(analysis.anomalies.is_empty());
        assert!(analysis.stable_periods.is_empty());
    }
}
