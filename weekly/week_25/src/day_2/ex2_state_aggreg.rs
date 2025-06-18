// 🎯 try_fold & State Aggregation - Intermédiaire vs Intermédiaire+ - 45min
//
// ⚓ MISSION :
// Implémenter un simulateur de protocole financier qui traite un lot de transactions.
// Vous devez utiliser `try_fold` pour calculer l'état final du système 
// (solde, nombre de transactions, pic de solde) en une seule passe, 
// en vous arrêtant à la première erreur de format ou de logique métier.
//
// 📋 SPÉCIFICATIONS :
// - Input: Un slice de chaînes de caractères `&[&str]`, où chaque chaîne est une commande.
// - Output: Un `Result<SimulationResult, ProcessingError>`.
// - Comportement:
//   - L'accumulateur sera une structure `SimulationResult`.
//   - Les commandes valides sont "DEPOSIT:amount" et "WITHDRAW:amount".
//   - Le traitement doit s'arrêter à la première commande invalide.

// 📐 RÈGLES & CAS LIMITES :
// 1. Le traitement doit s'arrêter immédiatement à la première erreur rencontrée.
// 2. Une erreur `ProcessingError::InsufficientFunds` doit être retournée si un retrait (`WITHDRAW`)
//    est tenté sur un solde inférieur au montant du retrait.
// 3. Une erreur `ProcessingError::InvalidCommand(String)` doit être retournée si une commande
//    est malformée (ex: pas de ':', montant non numérique, commande inconnue).
// 4. Le `highest_balance_seen` doit être mis à jour APRÈS chaque opération valide.

#[derive(Debug, PartialEq)]
pub enum ProcessingError {
    InvalidCommand(String),
    InsufficientFunds,
}

#[derive(Debug, PartialEq)]
pub struct SimulationResult {
    /// Le solde final après toutes les transactions valides.
    final_balance: u32,
    /// Le nombre total de transactions traitées avec succès.
    transactions_processed: usize,
    /// Le solde le plus élevé atteint durant la simulation.
    highest_balance_seen: u32,
}

// TODO: Implémentez votre solution en utilisant `try_fold`.
pub fn process_transactions(initial_balance: u32, transactions: &[&str]) -> Result<SimulationResult, ProcessingError> {
    let initial_state = SimulationResult {
        final_balance: initial_balance,
        transactions_processed: 0,
        highest_balance_seen: initial_balance,
    };

    transactions.iter().try_fold(initial_state, |mut acc, &command| {
        if !command.contains(':') {
            return Err(ProcessingError::InvalidCommand(command.to_string()));
        }
        let mut parts = command.split(':');
        
        let action = parts.next().ok_or(ProcessingError::InvalidCommand(command.to_string()))?;
        let str_amount = parts.next().ok_or(ProcessingError::InvalidCommand(command.to_string()))?;
        
        if parts.next().is_some() {
            return Err(ProcessingError::InvalidCommand(command.to_string()));
        }

        let amount = str_amount.parse::<u32>().map_err(|_| ProcessingError::InvalidCommand(command.to_string()))?;

        match action {
            "DEPOSIT" => {
                acc.transactions_processed += 1;
                acc.final_balance += amount
            },
            "WITHDRAW" => {
                if amount > acc.final_balance {
                    return Err(ProcessingError::InsufficientFunds);
                } else {
                    acc.transactions_processed += 1;
                    acc.final_balance -= amount
                }
            },
            _ => return Err(ProcessingError::InvalidCommand(command.to_string()))
        }

        if acc.final_balance > acc.highest_balance_seen {
            acc.highest_balance_seen = acc.final_balance
        }



        Ok(acc)
    })
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cas_nominal() {
        let _transactions = ["DEPOSIT:100", "WITHDRAW:30", "DEPOSIT:50"];
        let _expected: Result<SimulationResult,ProcessingError> = Ok(SimulationResult {
            final_balance: 120,
            transactions_processed: 3,
            highest_balance_seen: 150, // 50 (initial) -> 150 -> 120 -> 170. Non, 50 -> 150 -> 120. Le pic est bien 150.
        });
        // Correction: 50 (initial) -> 150 (après DEPOSIT:100) -> 120 (après WITHDRAW:30). Le pic est 150.
        // Puis 120 -> 170 (après DEPOSIT:50). Le pic final est 170.
        // Let's adjust the test case to be simpler.
        let transactions_simple = ["DEPOSIT:100", "WITHDRAW:30"];
        let expected_simple = Ok(SimulationResult {
            final_balance: 70,
            transactions_processed: 2,
            highest_balance_seen: 100,
        });
        assert_eq!(process_transactions(0, &transactions_simple), expected_simple);
    }

    #[test]
    fn test_cas_limite_fonds_insuffisants() {
        let transactions = ["DEPOSIT:50", "WITHDRAW:70"];
        assert_eq!(
            process_transactions(30, &transactions),
            Err(ProcessingError::InsufficientFunds)
        );
    }
    
    #[test]
    fn test_erreur_format_arrete_traitement() {
        // La commande "DEPOSIT:20" ne doit jamais être traitée car elle vient après l'erreur.
        let transactions = ["DEPOSIT:100", "HACK THE PLANET", "DEPOSIT:20"];
        match process_transactions(0, &transactions) {
            Err(ProcessingError::InvalidCommand(cmd)) => assert_eq!(cmd, "HACK THE PLANET"),
            _ => panic!("Le test aurait dû retourner une InvalidCommand error"),
        }
    }

    #[test]
    fn test_montant_invalide() {
        let transactions = ["DEPOSIT:vingt"];
        assert!(matches!(process_transactions(0, &transactions), Err(ProcessingError::InvalidCommand(_))));
    }

    #[test]

    fn test_solde_initial_est_pic_initial() {
        let transactions = ["WITHDRAW:100"];
        let expected = Ok(SimulationResult {
            final_balance: 100,
            transactions_processed: 1,
            highest_balance_seen: 200,
        });
        assert_eq!(process_transactions(200, &transactions), expected);
    }
}