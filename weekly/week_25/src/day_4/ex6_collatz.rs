pub struct Collatz {
    current: u64,
    is_finished: bool, // Indice: vous aurez peut-être besoin d'un état pour gérer la terminaison.
}

impl Collatz {
    /// Crée un nouvel itérateur de Collatz.
    pub fn new(start: u64) -> Self {
       Collatz { current: start, is_finished: false }
    }
}

impl Iterator for Collatz {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: Implémentez la logique de la séquence ici.
        // 1. Si la séquence est terminée, retournez toujours None.
        // 2. Si le nombre actuel est 1, marquez la séquence comme terminée pour le prochain appel.
        // 3. Retournez le nombre actuel.
        // 4. Calculez le prochain nombre et mettez à jour l'état interne (`self.current`).
        if self.current == 0 {
            return None;
        }

        if self.is_finished {
            return None;
        }
       
        let actual_number = self.current;
        if actual_number == 1 {
            self.is_finished = true;
            return Some(1);
        }

        if !self.is_finished {
            let next_number = if actual_number % 2 == 0 {
                actual_number / 2 
            } else {
                actual_number * 3 + 1
            };
            
            
            self.current = next_number;
        }
        

        Some(actual_number)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collatz_sequence_from_6() {
        let sequence: Vec<u64> = Collatz::new(6).collect();
        assert_eq!(sequence, vec![6, 3, 10, 5, 16, 8, 4, 2, 1]);
    }

    #[test]
    fn test_collatz_from_1() {
        let sequence: Vec<u64> = Collatz::new(1).collect();
        assert_eq!(sequence, vec![1]);
    }

    #[test]
    fn test_collatz_sum() {
        let sum: u64 = Collatz::new(10).map(|x| x * 2).sum();
        // Séquence: 10, 5, 16, 8, 4, 2, 1 -> sum = 46
        // Séquence * 2: 20, 10, 32, 16, 8, 4, 2 -> sum = 92
        assert_eq!(sum, 92);
    }

    #[test]
    fn test_empty_sequence_from_0() {
         // Par convention, une séquence démarrant à 0 ou moins n'est pas définie.
         // Notre `new` devrait peut-être gérer ça, mais pour cet exercice,
         // on peut s'attendre à ce que l'itérateur se termine immédiatement.
         // On va considérer que `new(0)` ne devrait pas paniquer.
         let sequence: Vec<u64> = Collatz::new(0).collect();
         assert_eq!(sequence, vec![]);
    }
}