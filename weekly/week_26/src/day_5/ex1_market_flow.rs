#[derive(Debug, PartialEq, Default)]
pub struct Trade {
    pub price: f64,
    pub volume: u64,
}

#[derive(Debug, PartialEq, Default)]
pub struct MarketDaySummary {
    /// Volume-Weighted Average Price
    pub vwap: f64,
    /// Cumulative volume traded after each trade
    pub cumulative_volumes: Vec<u64>,
    /// Biggest price drop between two consecutive trades
    pub biggest_price_drop: f64,
}

/// Analyzes a stream of trades and produces a summary.
/// The core of your work is to implement this function.
pub fn analyze_market_data(trades: impl Iterator<Item = Trade>) -> MarketDaySummary {
    let mut summary = MarketDaySummary::default();

    let trades = trades.collect::<Vec<Trade>>();
    if trades.is_empty() {
        return MarketDaySummary::default();
    }

    let scan = trades.iter().scan((0.0, 0), |(price, volume), trade| {
    
        *price = trade.price;
        *volume = trade.volume;

        Some((price.clone(), volume.clone()))
    });

    let (res, vol, vwap_up) = scan.fold(
        (Vec::new(), Vec::new(), 0.0),
        |(mut acc_res, mut acc_vol, mut vwap_up), (price, vol)| {
            acc_res.push(price);
            acc_vol.push(vol);
            vwap_up += price  * (vol as f64);

            (acc_res, acc_vol, vwap_up)
        },
    );

    summary.vwap = vwap_up / (vol.iter().sum::<u64>() as f64);

    let safe_window = &res[..res.len() - 1];
    summary.biggest_price_drop = safe_window
        .windows(2)
        .fold(0.0, |mut acc, price| {
            let window1 = price[0];
            let window2 = price[1];
            let delta = window2 - window1;
            if delta < acc {
                acc = delta;
            }
            acc
        })
        .abs();

    summary.cumulative_volumes = vol
        .iter()
        .scan(0, |state, vol| {
            *state += vol;
            Some(*state)
        })
        .collect::<Vec<u64>>();

    summary
}

#[cfg(test)]
mod tests {
    use super::*;

    // Permet des comparaisons de flottants approximatives
    fn f64_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn test_nominal_case() {
        let trades = vec![
            Trade {
                price: 100.0,
                volume: 10,
            },
            Trade {
                price: 102.0,
                volume: 5,
            },
            Trade {
                price: 99.0,
                volume: 20,
            },
            Trade {
                price: 101.0,
                volume: 15,
            },
        ];

        let summary = analyze_market_data(trades.into_iter());

        let expected_vwap = ((100.0 * 10.0) + (102.0 * 5.0) + (99.0 * 20.0) + (101.0 * 15.0))
            / (10.0 + 5.0 + 20.0 + 15.0); // 100.1
        let expected_volumes = vec![10, 15, 35, 50];
        let expected_drop = 3.0; // Entre 102.0 et 99.0

        assert!(
            f64_eq(summary.vwap, expected_vwap),
            "VWAP incorrect: attendu {}, obtenu {}",
            expected_vwap,
            summary.vwap
        );
        assert_eq!(
            summary.cumulative_volumes, expected_volumes,
            "Volumes cumulés incorrects"
        );
        assert!(
            f64_eq(summary.biggest_price_drop, expected_drop),
            "Baisse de prix incorrecte"
        );
    }

    #[test]
    fn test_empty_case() {
        let trades: Vec<Trade> = vec![];
        let summary = analyze_market_data(trades.into_iter());
        assert_eq!(summary, MarketDaySummary::default());
    }

    #[test]
    fn test_no_price_drop() {
        let trades = vec![
            Trade {
                price: 100.0,
                volume: 10,
            },
            Trade {
                price: 101.0,
                volume: 5,
            },
            Trade {
                price: 102.0,
                volume: 20,
            },
        ];
        let summary = analyze_market_data(trades.into_iter());
        assert!(
            f64_eq(summary.biggest_price_drop, 0.0),
            "La baisse de prix devrait être de 0 s'il n'y en a pas"
        );
    }
}
