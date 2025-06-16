// 🎯 Challenge: Somme Contrôlée
// 📊 Niveau: Medium
// ⏱️ Durée: 60min
//
// 📋 MISSION COMPLETE:
// Implémenter une fonction qui calcule la somme d'un slice de u32. La particularité
// est que la fonction doit gérer les dépassements de capacité (overflow). Si la somme
// à n'importe quel moment dépasse u32::MAX, la fonction doit immédiatement retourner None.
// L'outil parfait pour cela est try_fold combiné avec checked_add.
//
// 📥 ENTRÉES:
// - numbers: Un slice de u32 (&[u32]).
//
// 📤 SORTIES:
// - Success: Some(u32) contenant la somme totale si aucun overflow n'a eu lieu.
// - Erreur: None si la somme cumulative dépasse u32::MAX.
//
// 📏 RÈGLES MÉTIER:
// 1. La somme est initialisée à 0.
// 2. Pour chaque nombre, utiliser u32::checked_add pour l'ajouter à l'accumulateur.
// 3. checked_add retourne un Option, ce qui s'intègre naturellement avec try_fold.
//    try_fold s'arrêtera et propagera le None à la première occurrence d'un overflow.
// 4. Si le slice en entrée est vide, le résultat est Some(0).
//
// 🧪 TESTS FOURNIS:
// Couvrent le cas nominal, le cas vide, et le cas d'overflow au milieu du calcul.

pub fn checked_sum(numbers: &[u32]) -> Option<u32> {
    // Utilise numbers.iter().try_fold(...)
    // L'accumulateur est la somme courante (u32).
    // La closure prend l'accumulateur et le nombre suivant, et retourne le résultat de checked_add.
    if numbers.is_empty() {
        return Some(0);
    }
    numbers
        .iter()
        .try_fold(0u32, |acc, number| acc.checked_add(*number))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_without_overflow() {
        assert_eq!(checked_sum(&[10, 20, 30]), Some(60));
    }

    #[test]
    fn test_empty_slice_returns_zero() {
        assert_eq!(checked_sum(&[]), Some(0));
    }

    #[test]
    fn test_sum_with_overflow() {
        let numbers = vec![u32::MAX - 10, 10, 1]; // Dépassement à l'ajout de 1
        assert_eq!(checked_sum(&numbers), None);
    }

    #[test]
    fn test_sum_with_exact_max() {
        let numbers = vec![u32::MAX - 10, 10];
        assert_eq!(checked_sum(&numbers), Some(u32::MAX));
    }

    #[test]
    fn test_single_large_number() {
        assert_eq!(checked_sum(&[u32::MAX]), Some(u32::MAX));
    }
}

// 6 minutes
