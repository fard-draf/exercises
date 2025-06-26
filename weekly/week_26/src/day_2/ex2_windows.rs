// main.rs ou lib.rs

/// Calcule la moyenne mobile d'une série de données.
///
/// # Arguments
/// * `data` - Une slice de f64 représentant la série de données.
/// * `window_size` - La taille de la fenêtre glissante pour calculer la moyenne.
///
/// # Retourne
/// Un vecteur de f64 contenant les moyennes mobiles.
pub fn moving_average(data: &[f64], window_size: usize) -> Vec<f64> {
    data.windows(window_size)
        .map(|(e)| e.iter().sum::<f64>() / window_size as f64)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Précision pour la comparaison des f64
    const F64_PRECISION: f64 = 1e-9;

    fn assert_f64_vec_eq(a: &[f64], b: &[f64]) {
        assert_eq!(a.len(), b.len(), "Les vecteurs n'ont pas la même longueur");
        for (i, (val_a, val_b)) in a.iter().zip(b.iter()).enumerate() {
            assert!(
                (val_a - val_b).abs() < F64_PRECISION,
                "Différence à l'index {}: {} vs {}",
                i,
                val_a,
                val_b
            );
        }
    }

    #[test]
    fn test_moving_average_nominal() {
        let data = vec![10.0, 12.0, 11.0, 15.0, 16.0, 14.0, 13.0];
        let window_size = 3;
        let expected = vec![11.0, 12.666666667, 14.0, 15.0, 14.333333333];
        let result = moving_average(&data, window_size);
        assert_f64_vec_eq(&result, &expected);
    }

    #[test]
    fn test_data_shorter_than_window() {
        let data = vec![10.0, 20.0];
        let window_size = 3;
        let result = moving_average(&data, window_size);
        assert!(
            result.is_empty(),
            "Le résultat devrait être vide si les données sont plus courtes que la fenêtre."
        );
    }

    #[test]
    fn test_window_size_one() {
        let data = vec![10.0, 12.0, 11.0];
        let window_size = 1;
        let expected = vec![10.0, 12.0, 11.0];
        let result = moving_average(&data, window_size);
        assert_f64_vec_eq(&result, &expected);
    }

    #[test]
    fn test_empty_data() {
        let data: Vec<f64> = vec![];
        let window_size = 3;
        let result = moving_average(&data, window_size);
        assert!(
            result.is_empty(),
            "Le résultat devrait être vide si les données sont vides."
        );
    }

    #[test]
    fn test_data_equals_window_size() {
        let data = vec![1.0, 2.0, 3.0];
        let window_size = 3;
        let expected = vec![2.0];
        let result = moving_average(&data, window_size);
        assert_f64_vec_eq(&result, &expected);
    }
}
