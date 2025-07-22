pub fn process_transactions(
    initial_balance: f64,
    overdraft_limit: f64,
    transactions: impl Iterator<Item = f64>,
) -> impl Iterator<Item = f64> {
    // La signature de la closure de scan est `|&mut state, item| -> Option<Output>`.
    // - `state` sera ton solde.
    // - `item` sera la transaction.
    // - `Option<Output>` est le véhicule pour continuer (`Some`) ou arrêter (`None`) l'itération.

    transactions.scan(initial_balance, move |balance, transaction| {
        if *balance <= overdraft_limit {
            return None;
        }

        *balance += transaction;

        if *balance <= overdraft_limit {
            return None;
        }

        Some(*balance)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_operation() {
        let transactions = vec![100.0, -50.0, 25.0];
        let history: Vec<_> = process_transactions(0.0, -200.0, transactions.into_iter()).collect();
        assert_eq!(history, vec![100.0, 50.0, 75.0]);
    }

    #[test]
    fn test_overdraft_stops_iteration() {
        let transactions = vec![50.0, -100.0, -100.0, 5000.0]; // La dernière transaction ne sera jamais atteinte
        let history: Vec<_> =
            process_transactions(100.0, -50.0, transactions.into_iter()).collect();
        // 1. 100 + 50 = 150
        // 2. 150 - 100 = 50
        // 3. 50 - 100 = -50 -> Limite atteinte. On arrête.
        assert_eq!(history, vec![150.0, 50.0]);
    }

    #[test]
    fn test_empty_transactions() {
        let transactions: Vec<f64> = vec![];
        let history: Vec<_> =
            process_transactions(100.0, -100.0, transactions.into_iter()).collect();
        assert!(history.is_empty());
    }

    #[test]
    fn test_overdraft_on_first_transaction() {
        let transactions = vec![-200.0, 50.0];
        let history: Vec<_> =
            process_transactions(100.0, -150.0, transactions.into_iter()).collect();
        // 1. 100 - 200 = -100. C'est OK.
        // La limite est à -150, donc -100 est > -150.
        assert_eq!(history, vec![-100.0, -50.00]);
    }

    #[test]
    fn test_overdraft_on_first_transaction_and_stops() {
        let transactions = vec![-200.0, 50.0];
        let history: Vec<_> = process_transactions(0.0, -150.0, transactions.into_iter()).collect();
        // 1. 0 - 200 = -200 -> Limite dépassée. On arrête et on ne produit rien.
        assert!(history.is_empty());
    }
}
