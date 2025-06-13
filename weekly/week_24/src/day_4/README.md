🎯 Exercice 1 : Fold Numérique - Machine à États Simple
📊 Niveau : Easy
⏱️ Durée : 45min
📋 CONTEXTE COMPLET :
Implémenter des fonctions utilisant fold pour comprendre le pattern d'accumulation sur des nombres. Chaque fonction doit utiliser UNIQUEMENT fold (pas de boucles, pas d'autres iterators).
📥 ENTRÉES :

numbers: &[i32] - Slice d'entiers signés
Contraintes :

Peut être vide
Valeurs entre -10_000 et 10_000
Maximum 1000 éléments



📤 SORTIES :

sum_fold() → i32 (0 si vide)
product_fold() → i32 (1 si vide)
min_max_fold() → Option<(i32, i32)> (None si vide)
running_average_fold() → Vec<f64> (moyennes cumulatives)

📏 RÈGLES MÉTIER :

sum_fold : Additionne tous les éléments, retourne 0 pour slice vide
product_fold : Multiplie tous les éléments, retourne 1 pour slice vide
min_max_fold : Retourne Some((min, max)) ou None si vide
running_average_fold : Pour chaque position i, retourne la moyenne des éléments [0..=i]

Exemple : [10, 20, 30] → [10.0, 15.0, 20.0]
Precision f64, pas d'arrondi



💻 CODE :
rust// 🎯 Challenge: Fold Numerical States
// 📊 Niveau: Easy
// ⏱️ Durée: 45min

pub fn sum_fold(numbers: &[i32]) -> i32 {
    // TODO: Utiliser fold pour calculer la somme
    // Accumulateur initial: 0
    // État: acc + element
    todo!()
}

pub fn product_fold(numbers: &[i32]) -> i32 {
    // TODO: Utiliser fold pour calculer le produit
    // Accumulateur initial: 1
    // État: acc * element
    todo!()
}

pub fn min_max_fold(numbers: &[i32]) -> Option<(i32, i32)> {
    // TODO: Utiliser fold pour trouver min ET max en un seul passage
    // Accumulateur: Option<(min, max)>
    // État: mise à jour simultanée min/max
    todo!()
}

pub fn running_average_fold(numbers: &[i32]) -> Vec<f64> {
    // TODO: Utiliser fold pour calculer les moyennes cumulatives
    // Accumulateur: (Vec<f64>, somme_courante, count)
    // État: ajout moyenne dans vec + mise à jour somme/count
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_fold() {
        assert_eq!(sum_fold(&[1, 2, 3, 4, 5]), 15);
        assert_eq!(sum_fold(&[-5, 5, -3, 3]), 0);
        assert_eq!(sum_fold(&[]), 0);
        assert_eq!(sum_fold(&[42]), 42);
    }

    #[test]
    fn test_product_fold() {
        assert_eq!(product_fold(&[2, 3, 4]), 24);
        assert_eq!(product_fold(&[-2, 3, -1]), 6);
        assert_eq!(product_fold(&[]), 1);
        assert_eq!(product_fold(&[0, 100, 200]), 0);
    }

    #[test]
    fn test_min_max_fold() {
        assert_eq!(min_max_fold(&[3, 1, 4, 1, 5]), Some((1, 5)));
        assert_eq!(min_max_fold(&[42]), Some((42, 42)));
        assert_eq!(min_max_fold(&[-10, 0, 10]), Some((-10, 10)));
        assert_eq!(min_max_fold(&[]), None);
    }

    #[test]
    fn test_running_average_fold() {
        let result = running_average_fold(&[10, 20, 30]);
        assert_eq!(result, vec![10.0, 15.0, 20.0]);
        
        let result = running_average_fold(&[5]);
        assert_eq!(result, vec![5.0]);
        
        let result = running_average_fold(&[]);
        assert_eq!(result, vec![]);
        
        let result = running_average_fold(&[1, 2, 3, 4, 5]);
        assert_eq!(result, vec![1.0, 1.5, 2.0, 2.5, 3.0]);
    }
}
💡 Indices :

Fold signature : iter.fold(initial, |acc, item| new_acc)
Pour min_max : penser à match sur l'accumulateur Option
Pour running_average : l'accumulateur peut être un tuple complexe