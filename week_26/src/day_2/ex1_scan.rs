// main.rs ou lib.rs

/// Calcule l'historique des soldes d'un compte après une série de transactions.
///
/// # Arguments
/// * `initial_balance` - Le solde de départ du compte.
/// * `transactions` - Un itérateur sur les montants des transactions.
///
/// # Retourne
/// Un itérateur produisant le solde après chaque transaction.
pub fn account_history(
    initial_balance: f64,
    transactions: impl Iterator<Item = f64>,
) -> impl Iterator<Item = f64> {
    transactions.scan(initial_balance, |state, transaction| {
        *state += transaction;

        Some(*state)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nominal_case() {
        let initial_balance = 100.0;
        let transactions = vec![15.5, -30.0, 5.25];
        let history: Vec<f64> =
            account_history(initial_balance, transactions.into_iter()).collect();
        assert_eq!(history, vec![115.5, 85.5, 90.75]);
    }

    #[test]
    fn test_empty_transactions() {
        let initial_balance = 50.0;
        let transactions: Vec<f64> = vec![];
        let history: Vec<f64> =
            account_history(initial_balance, transactions.into_iter()).collect();
        assert!(
            history.is_empty(),
            "L'historique devrait être vide s'il n'y a pas de transactions."
        );
    }

    #[test]
    fn test_negative_balance() {
        let initial_balance = 20.0;
        let transactions = vec![-10.0, -15.0, 5.0]; // Le solde passe à -5.0
        let history: Vec<f64> =
            account_history(initial_balance, transactions.into_iter()).collect();
        assert_eq!(history, vec![10.0, -5.0, 0.0]);
    }

    #[test]
    fn test_zero_initial_balance() {
        let initial_balance = 0.0;
        let transactions = vec![50.0, -25.0];
        let history: Vec<f64> =
            account_history(initial_balance, transactions.into_iter()).collect();
        assert_eq!(history, vec![50.0, 25.0]);
    }
}
