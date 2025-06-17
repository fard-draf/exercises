use std::{collections::HashMap, io::empty};

/// Parse une chaîne de requête `&str` en `Result<HashMap<String, u32>, String>`.
///
/// La fonction utilise `try_fold` pour construire la HashMap de manière faillible,
/// en s'arrêtant à la première erreur de format ou de parsing.
fn solution(input: &str) -> Result<HashMap<String, u32>, String> {
    // TODO: Votre implémentation commence ici.
    // L'objectif est de faire passer tous les tests unitaires ci-dessous.

    let slices = input.split('&').collect::<Vec<_>>();
    slices
        .iter()
        .try_fold(HashMap::<String, u32>::new(), |mut acc, slice| {
            if slice.is_empty() {
                return Ok(HashMap::new());
            }
            let mut chunk = slice.split('=');

            let key = chunk.next().ok_or("Unvalid".to_string())?;
            let value = chunk.next().ok_or("Unvalid".to_string())?;

            if key.is_empty() {
                return Err("Empty key".to_string());
            }
            if value.is_empty() || value.chars().all(|e| !e.is_numeric()) {
                return Err("Unvalid value".to_string());
            }

            let value = value
                .parse::<u32>()
                .map_err(|_| "Unable to parse".to_string())?;

            acc.insert(key.to_string(), value);
            Ok(acc)
        }) 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cas_nominal() {
        let query = "id=10&power=9000&user=1";
        let mut expected = HashMap::new();
        expected.insert("id".to_string(), 10);
        expected.insert("power".to_string(), 9000);
        expected.insert("user".to_string(), 1);
        assert_eq!(solution(query), Ok(expected));
    }

    #[test]
    fn test_chaine_vide() {
        assert_eq!(solution(""), Ok(HashMap::new()));
    }

    #[test]
    fn test_cle_dupliquee() {
        let query = "a=1&b=2&a=3";
        let mut expected = HashMap::new();
        expected.insert("b".to_string(), 2);
        expected.insert("a".to_string(), 3);
        assert_eq!(solution(query), Ok(expected));
    }

    #[test]
    fn test_erreur_parsing_valeur() {
        let query = "id=10&power=invalid&user=1";
        assert!(solution(query).is_err());
    }

    #[test]
    fn test_erreur_segment_malforme_sans_egal() {
        let query = "id=10&malformed&user=1";
        assert!(solution(query).is_err());
    }

    #[test]
    fn test_erreur_cle_vide() {
        let query = "id=10&=123";
        assert!(solution(query).is_err());
    }

    #[test]
    fn test_erreur_valeur_vide() {
        let query = "id=10&key=&user=1";
        assert!(solution(query).is_err());
    }
}
