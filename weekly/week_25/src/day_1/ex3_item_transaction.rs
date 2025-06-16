// 🎯 Challenge: Processeur de Transactions par Lot
// 📊 Niveau: Medium
// ⏱️ Durée: 1h
//
// 📋 MISSION COMPLETE:
// Tu es chargé de valider un lot de transactions financières. Une banque veut s'assurer
// qu'un lot complet de transactions peut être appliqué à un compte sans que son solde
// ne devienne jamais négatif. Si une seule transaction du lot est invalide (c.-à-d.
// un retrait qui passerait le solde sous zéro), l'intégralité du lot doit être rejetée.
// Ta mission est d'utiliser `try_fold` pour calculer le solde final si le lot est valide,
// ou de t'arrêter et de retourner une erreur à la première transaction invalide.
//
// 📥 ENTRÉES:
// - `initial_balance`: u64, le solde de départ du compte.
// - `transactions`: Un slice de `Transaction` (`&[Transaction]`).
//
// 📤 SORTIES:
// - `Success`: `Ok(u64)`, contenant le solde final si toutes les transactions ont été traitées avec succès.
// - `Erreur`: `Err(ProcessingError)`, si une transaction échoue.
//
// 📏 RÈGLES MÉTIER:
// 1. L'accumulateur pour `try_fold` sera le solde du compte, en commençant par `initial_balance`.
// 2. Pour une `Transaction::Deposit(amount)`, le nouveau solde est `solde_actuel + amount`.
// 3. Pour une `Transaction::Withdrawal(amount)`, le retrait n'est valide que si `amount <= solde_actuel`.
// 4. Si un retrait est invalide, la fonction doit immédiatement retourner `Err(ProcessingError::InsufficientFunds)`.
// 5. Si le slice de transactions est vide, la fonction doit retourner `Ok(initial_balance)`.
//
// 💡 INDICE:
// La closure de ton `try_fold` prendra le solde courant (`acc`) et une transaction,
// et devra retourner un `Result<u64, ProcessingError>` (le nouveau solde ou une erreur).

pub enum Transaction {
    Deposit(u64),
    Withdrawal(u64),
}

#[derive(Debug, PartialEq)]
pub enum ProcessingError {
    InsufficientFunds { required: u64, available: u64 },
    UnaviableTransaction,
}

/// Processe un lot de transactions et retourne le solde final.
/// Échoue si une transaction ferait passer le solde sous zéro.
pub fn process_transaction_batch(
    initial_balance: u64,
    transactions: &[Transaction],
) -> Result<u64, ProcessingError> {
    transactions
        .iter()
        .try_fold(initial_balance, |mut acc, transaction| match transaction {
            Transaction::Deposit(value) => {
                if *value > 0 {
                    acc += value;
                    Ok(acc)
                } else {
                    Err(ProcessingError::UnaviableTransaction)
                }
            }
            Transaction::Withdrawal(value) => {
                if acc >= *value {
                    acc -= value;
                    Ok(acc)
                } else {
                    Err(ProcessingError::InsufficientFunds {
                        required: *value,
                        available: acc,
                    })
                }
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_batch_of_transactions() {
        let transactions = vec![
            Transaction::Deposit(100),   // Solde: 200
            Transaction::Withdrawal(50), // Solde: 150
            Transaction::Deposit(20),    // Solde: 170
        ];
        assert_eq!(process_transaction_batch(100, &transactions), Ok(170));
    }

    #[test]
    fn test_batch_fails_on_insufficient_funds() {
        let transactions = vec![
            Transaction::Deposit(100),    // Solde: 200
            Transaction::Withdrawal(250), // Doit échouer ici
            Transaction::Deposit(50),     // Ne doit jamais être atteint
        ];
        assert_eq!(
            process_transaction_batch(100, &transactions),
            Err(ProcessingError::InsufficientFunds {
                required: 250,
                available: 200
            })
        );
    }

    #[test]
    fn test_empty_batch_returns_initial_balance() {
        let transactions = vec![];
        assert_eq!(process_transaction_batch(500, &transactions), Ok(500));
    }

    #[test]
    fn test_exact_balance_withdrawal_is_ok() {
        let transactions = vec![Transaction::Withdrawal(100)];
        assert_eq!(process_transaction_batch(100, &transactions), Ok(0));
    }

    #[test]
    fn test_withdrawal_to_zero_then_fail() {
        let transactions = vec![
            Transaction::Withdrawal(100), // Solde: 0
            Transaction::Withdrawal(1),   // Doit échouer ici
        ];
        assert_eq!(
            process_transaction_batch(100, &transactions),
            Err(ProcessingError::InsufficientFunds {
                required: 1,
                available: 0
            })
        );
    }
}
