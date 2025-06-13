// 📋 CONTEXTE COMPLET:
// Chaque fonction utilise fold avec un pattern d'accumulateur différent
// Objectif: Automatiser le choix du bon accumulateur selon le besoin

use std::collections::HashMap;

// 1. Accumulator: (T, usize) - valeur + compteur
fn average(numbers: &[i32]) -> Option<f64> {
    // Si vide -> None
    // Sinon -> Some(moyenne en f64)
    // Pattern: fold avec (sum, count) puis division finale
    if numbers.is_empty() {
        return None;
    }

    let sum = numbers.iter().fold(None, |mut acc, &nbr| {
        match acc {
            None => acc = Some((nbr, 1)),
            Some((mut sum, mut count)) => {
                sum += nbr;
                count += 1;
                acc = Some((sum, count))
            }
        };
        acc
    });

    if let Some(value) = sum {
        Some(value.0 as f64 / value.1 as f64)
    } else {
        None
    }
}

// 2. Accumulator: Option<(T, T)> - min/max simultané
fn bounds(values: &[i32]) -> Option<(i32, i32)> {
    // Si vide -> None
    // Sinon -> Some((min, max))
    // Un seul passage, pas deux fold séparés
    if values.is_empty() {
        return None;
    }

    values.iter().fold(None, |mut acc, &nbr| {
        match acc {
            None => acc = Some((nbr, nbr)),
            Some((min, max)) => acc = Some((min.min(nbr), max.max(nbr))),
        };
        acc
    })
}

// 3. Accumulator: HashMap<K, V> - grouping
fn group_by_length(words: &[&str]) -> HashMap<usize, Vec<String>> {
    // Grouper les mots par leur longueur
    // Ex: ["hi", "bye", "test"] -> {2: ["hi"], 3: ["bye"], 4: ["test"]}

    words
        .iter()
        .fold(HashMap::<usize, Vec<String>>::new(), |mut acc, &word| {
            acc.entry(word.len()).or_default().push(word.to_string());
            acc
        })
}

// 4. Accumulator: (Vec<T>, Predicate) - partitioning
fn partition_fold<T: Clone>(items: &[T], predicate: fn(&T) -> bool) -> (Vec<T>, Vec<T>) {
    // Séparer en (matching, not_matching)
    // Un seul fold, pas filter().collect() deux fois
    items
        .iter()
        .fold((Vec::new(), Vec::new()), |mut acc, item| {
            match predicate(item) {
                false => acc.1.push(item.clone()),
                true => acc.0.push(item.clone()),
            };
            acc
        })
}

// 5. Accumulator: Result<T, E> - early termination
fn validate_all(numbers: &[i32]) -> Result<Vec<i32>, &'static str> {
    // Si tous >= 0 -> Ok(vec des nombres)
    // Si un négatif -> Err("negative found")
    // Arrêt dès première erreur
    let result = numbers
        .iter()
        .fold((Vec::<i32>::new(), 0), |mut acc, &nbr| {
            match nbr {
                value if value >= 0 => acc.0.push(value),
                _ => acc.1 += 1,
            };

            if acc.1 > 0 {
                return acc;
            }
            acc
        });

    if result.1 > 0 {
        Err("negative found")
    } else {
        Ok(result.0)
    }
}

// 6. Accumulator: (bool, State) - state machine simple
fn is_alternating(values: &[i32]) -> bool {
    // Vérifier si les valeurs alternent entre pair/impair
    // [2,3,4,5] -> true (pair,impair,pair,impair)
    // [2,4,3] -> false (pair,pair,...)
    // Vide ou 1 élément -> true

    values.windows(2).all(|w| w[0] % 2 != w[1] % 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_average() {
        assert_eq!(average(&[]), None);
        assert_eq!(average(&[5]), Some(5.0));
        assert_eq!(average(&[1, 2, 3, 4]), Some(2.5));
    }

    #[test]
    fn test_bounds() {
        assert_eq!(bounds(&[]), None);
        assert_eq!(bounds(&[5]), Some((5, 5)));
        assert_eq!(bounds(&[3, 1, 4, 1, 5]), Some((1, 5)));
    }

    #[test]
    fn test_group_by_length() {
        let words = vec!["hi", "bye", "test", "ok"];
        let groups = group_by_length(&words);
        assert_eq!(groups[&2], vec!["hi", "ok"]);
        assert_eq!(groups[&3], vec!["bye"]);
        assert_eq!(groups[&4], vec!["test"]);
    }

    #[test]
    fn test_partition() {
        let nums = vec![1, 2, 3, 4, 5];
        let (even, odd) = partition_fold(&nums, |n| n % 2 == 0);
        assert_eq!(even, vec![2, 4]);
        assert_eq!(odd, vec![1, 3, 5]);
    }

    #[test]
    fn test_validate() {
        assert_eq!(validate_all(&[1, 2, 3]), Ok(vec![1, 2, 3]));
        assert_eq!(validate_all(&[1, -2, 3]), Err("negative found"));
    }

    #[test]
    fn test_alternating() {
        assert!(is_alternating(&[]));
        assert!(is_alternating(&[1]));
        assert!(is_alternating(&[2, 3, 4, 5]));
        assert!(!is_alternating(&[2, 4, 3]));
    }
}
