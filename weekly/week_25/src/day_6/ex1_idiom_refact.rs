// La version à battre (ne pas modifier)
pub fn find_balance_dips_imperative(
    transactions: &[i32],
    initial_balance: i32,
    threshold: i32,
) -> Vec<(usize, i32)> {
    let mut results = Vec::new();
    let mut current_balance = initial_balance;
    let mut was_below_threshold = current_balance < threshold;

    for (i, &transaction) in transactions.iter().enumerate() {
        current_balance += transaction;
        let is_below_threshold = current_balance < threshold;

        // On signale uniquement la transition de "au-dessus" à "en-dessous"
        if is_below_threshold && !was_below_threshold {
            results.push((i, current_balance));
        }
        
        was_below_threshold = is_below_threshold;
    }
    results
}

// C'est ici que tu travailles !
pub fn find_balance_dips_idiomatic(
    transactions: &[i32],
    initial_balance: i32,
    threshold: i32,
) -> Vec<(usize, i32)> {
    // TODO: Réimplémente la logique en utilisant `scan`, `filter_map`, et `collect`.
    // L'état de ton `scan` devrait probablement être un tuple pour suivre le solde
    // et l'état précédent (au-dessus/en-dessous du seuil).
    let current_balance = initial_balance;
    let was_below_threshold = current_balance < threshold;


  transactions.iter().enumerate().scan((was_below_threshold, current_balance), |states, operation| {

        states.1 += operation.1;       

        // saving  current state
        let previous_was = states.0;    
        
        //update for next state
        states.0 = states.1 < threshold;
        
        Some((operation.0, states.0, previous_was, states.1))


        
        
        
    }).filter_map(|(index, is_below, was_below, balance) | {
        println!("{} is-{}, was-{}, {}", index, is_below, was_below, balance );
        if is_below && !was_below {
            Some((index , balance))
        } else {
            None
        }
    }).collect::<Vec<(usize, i32)>>()


    



    


    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dip_detection() {
        let transactions = &[10, -20, -100, 50, -30];
        let initial_balance = 100;
        let threshold = 10;
        // Balance steps: 100 -> 110 -> 90 -> -10 (dip!) -> 40 -> 10
        let expected = vec![(2, -10)];
        assert_eq!(find_balance_dips_idiomatic(transactions, initial_balance, threshold), expected);
    }

    #[test]
    fn test_multiple_dips() {
        let transactions = &[-90, 100, -90]; // Balances: 10, 110, 20
        let initial_balance = 100;
        let threshold = 30;
        let expected = vec![(0, 10), (2, 20)];
        assert_eq!(find_balance_dips_idiomatic(transactions, initial_balance, threshold), expected);
    }
    
    #[test]
    fn test_starts_below() {
        let transactions = &[-10, -20]; // Balances: -5, -25
        let initial_balance = 5;
        let threshold = 10;
        // Le premier état sous le seuil est après la première transaction.
        let expected = vec![(0, -5)];
        assert_eq!(find_balance_dips_idiomatic(transactions, initial_balance, threshold), expected);
    }
}