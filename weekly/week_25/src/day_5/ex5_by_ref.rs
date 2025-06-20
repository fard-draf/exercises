fn consume_first_five_even(ref_iter: &mut impl Iterator<Item = u32>) -> Vec<u32> {
    ref_iter.by_ref().filter(|e| e % 2 == 0).take(5).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_consume_first_five_even() {
        // Objectif final d'utilisation
        let mut numbers = (0..=20).into_iter(); // Un itérateur de 0 à 20

        // On consomme les 5 premiers nombres pairs
        let first_five_even = consume_first_five_even(&mut numbers);
        assert_eq!(first_five_even, vec![0, 2, 4, 6, 8]);

        // L'itérateur original `numbers` a été avancé.
        // Il doit maintenant commencer APRÈS le dernier nombre consommé (le 8).
        // Le prochain élément doit donc être 9.
        assert_eq!(numbers.next(), Some(9));
        assert_eq!(numbers.next(), Some(10));
    }
}
