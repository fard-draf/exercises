// 🎯 Challenge: Statistical Fold Patterns
// 📊 Niveau: Medium (Progressif)
// ⏱️ Durée: 1h

use core::num;

#[derive(Debug, PartialEq)]
pub struct Stats {
    pub min: f64,
    pub max: f64,
    pub mean: f64,
}

// Niveau 1 : Simple moyenne
pub fn mean(numbers: &[f64]) -> Option<f64> {
    // Retourne None si vide
    // Sinon la moyenne
    if numbers.is_empty() {
        return None;
    }
    let total = numbers.iter().fold(0.0, |acc, nbr| acc + *nbr);
    Some(total / numbers.len() as f64)
}

// Niveau 2 : Variance en un passage (astuce mathématique)
pub fn variance(numbers: &[f64]) -> Option<f64> {
    // Variance = E[X²] - (E[X])²
    // Donc on peut calculer somme et somme_des_carrés en un passage
    // Retourne None si vide
    if numbers.is_empty() {
        return None;
    }

    let totals = numbers
        .iter()
        .fold((0.0, 0.0, 0), |(mut sum, mut sum_sqr, mut count), &nbr| {
            sum += nbr;
            sum_sqr += nbr.powi(2);
            count += 1;
            (sum, sum_sqr, count)
        });

    let variance = (totals.1 / totals.2 as f64) - (totals.0 / totals.2 as f64).powi(2);

    Some(variance)
}

// Niveau 3 : Multiple stats simultanées
pub fn min_max_mean(numbers: &[f64]) -> Option<Stats> {
    // Calculer min, max et mean en UN SEUL passage
    // None si vide
    if numbers.is_empty() {
        return None;
    }

    let totals = numbers
        .iter()
        .fold((0.0, 0.0, 0.0, 0), |(acc), &nbr| match acc {
            (0.0, 0.0, 0.0, 0) => (nbr, nbr, nbr, 1),
            (min, max, mut mean, mut count) => {
                count += 1;
                mean += nbr;
                (min.min(nbr), max.max(nbr), mean, count)
            }
        });

    struct Stat {
        min: f64,
        max: f64,
        mean: f64,
    }

    Some(Stats {
        min: totals.0,
        max: totals.1,
        mean: totals.2 / totals.3 as f64,
    })
}

// Niveau 4 : Médiane (nécessite de trier)
pub fn median(numbers: &[f64]) -> Option<f64> {
    // D'abord collecter dans un Vec avec fold
    // Puis trier et trouver la médiane
    // None si vide

    if numbers.is_empty() {
        return None;
    }

    let mut vector = numbers
        .iter()
        .fold(Vec::with_capacity(numbers.len()), |mut acc, &nbr| {
            acc.push(nbr);
            acc
        });

    vector.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = numbers.len() / 2;

    println!("{:?}", vector);

    if numbers.len() % 2 == 1 {
        Some(vector[mid])
    } else {
        Some((vector[mid - 1] + vector[mid]) / 2.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mean() {
        assert_eq!(mean(&[1.0, 2.0, 3.0, 4.0, 5.0]), Some(3.0));
        assert_eq!(mean(&[10.0]), Some(10.0));
        assert_eq!(mean(&[-5.0, 5.0]), Some(0.0));
        assert_eq!(mean(&[]), None);
    }

    #[test]
    fn test_variance() {
        // Variance de [1,2,3,4,5] = 2.0
        let result = variance(&[1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();
        assert!((result - 2.0).abs() < 0.0001);

        // Variance de valeurs identiques = 0
        assert_eq!(variance(&[5.0, 5.0, 5.0]), Some(0.0));

        assert_eq!(variance(&[]), None);
    }

    #[test]
    fn test_min_max_mean() {
        let result = min_max_mean(&[1.0, 5.0, 3.0, 2.0, 4.0]).unwrap();
        assert_eq!(result.min, 1.0);
        assert_eq!(result.max, 5.0);
        assert_eq!(result.mean, 3.0);

        let result = min_max_mean(&[42.0]).unwrap();
        assert_eq!(result.min, 42.0);
        assert_eq!(result.max, 42.0);
        assert_eq!(result.mean, 42.0);

        assert_eq!(min_max_mean(&[]), None);
    }

    #[test]
    fn test_median() {
        // Nombre impair d'éléments
        assert_eq!(median(&[3.0, 1.0, 5.0, 2.0, 4.0]), Some(3.0));

        // Nombre pair d'éléments
        assert_eq!(median(&[1.0, 2.0, 3.0, 4.0]), Some(2.5));

        // Un seul élément
        assert_eq!(median(&[42.0]), Some(42.0));

        assert_eq!(median(&[]), None);
    }
}
