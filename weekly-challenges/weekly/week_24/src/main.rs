// 🎯 Challenge: Fold Numerical States
// 📊 Niveau: Easy
// ⏱️ Durée: 45min

use core::num;

pub fn sum_fold(numbers: &[i32]) -> i32 {
    // TODO: Utiliser fold pour calculer la somme
    // Accumulateur initial: 0
    // État: acc + element

    numbers.iter().fold(0i32, |acc, nbr| acc + *nbr)
}

pub fn product_fold(numbers: &[i32]) -> i32 {
    // TODO: Utiliser fold pour calculer le produit
    // Accumulateur initial: 1
    // État: acc * element
    numbers.iter().fold(1i32, |mut acc, &nbr| {
        acc *= nbr;
        acc
    })
}

pub fn min_max_fold(numbers: &[i32]) -> Option<(i32, i32)> {
    // TODO: Utiliser fold pour trouver min ET max en un seul passage
    // Accumulateur: Option<(min, max)>
    // État: mise à jour simultanée min/max
    numbers.iter().fold(None, |acc, &nbr| match acc {
        None => Some((nbr, nbr)),
        Some((min, max)) => Some((min.min(nbr), max.max(nbr))),
    })
}

pub fn running_average_fold(numbers: &[i32]) -> Vec<f64> {
    // TODO: Utiliser fold pour calculer les moyennes cumulatives
    // Accumulateur: (Vec<f64>, somme_courante, count)
    // État: ajout moyenne dans vec + mise à jour somme/count
    let (_, _, result) =
        numbers
            .iter()
            .fold((0, 0.0, Vec::new()), |(mut count, sum, mut vec), &nbr| {
                count += 1;
                let new_sum = sum + nbr as f64;
                let average = new_sum / (count) as f64;
                vec.push(average);
                (count, new_sum, vec)
            });
    result
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
