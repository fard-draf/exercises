// 🎯 Challenge: Parseur de Chaîne de Nombres
// 📊 Niveau: Medium
// ⏱️ Durée: 1h 30min
//
// 📋 MISSION COMPLETE:
// Tu dois écrire une fonction qui parse une chaîne de caractères contenant des nombres
// entiers (`i32`) séparés par des virgules. La fonction doit retourner un vecteur
// de ces nombres si toutes les parties sont valides. Si une seule partie de la
// chaîne n'est pas un nombre valide, la fonction doit immédiatement échouer et
// retourner une erreur descriptive.
//
// 📥 ENTRÉES:
// - `s`: Une chaîne de caractères `&str`, par exemple "1,2,-5, 10".
//
// 📤 SORTIES:
// - `Success`: `Ok(Vec<i32>)`, contenant la liste des nombres parsés.
// - `Erreur`: `Err(String)`, avec un message expliquant quel élément a échoué.
//
// 📏 RÈGLES MÉTIER:
// 1. La chaîne doit être divisée en sous-chaînes sur le délimiteur `,`.
// 2. Chaque sous-chaîne doit être "nettoyée" : les espaces blancs au début et à la fin doivent être retirés.
// 3. Après nettoyage, si une sous-chaîne est vide (ex: "1,,2"), elle doit être ignorée.
// 4. Chaque sous-chaîne non-vide doit être parsée en `i32`.
// 5. Si le parsing d'un seul élément échoue, toute l'opération s'arrête.
//    L'erreur retournée doit être une `String` du type : "Erreur de parsing sur l'élément '{element_fautif}'".
// 6. Une chaîne d'entrée vide ou ne contenant que des espaces et/ou des virgules doit résulter en un `Ok` avec un vecteur vide.
//
// 💡 INDICE:
// C'est le moment d'utiliser un des patterns les plus puissants et idiomatiques de Rust.
// Après avoir utilisé `.split()` et `.map()` pour transformer chaque sous-chaîne en `Result<i32, _>`,
// tu te retrouveras avec un itérateur de `Result`s.
// Tu peux appeler `.collect()` DIRECTEMENT sur cet itérateur !
// Rust peut transformer un `Iterator<Item = Result<T, E>>` en `Result<Vec<T>, E>`.
// C'est le "pattern collect sur Result". Il s'arrête à la première erreur, exactement comme `try_fold`.

/// Parse une chaîne de nombres séparés par des virgules en un `Vec<i32>`.
/// Échoue si un des éléments n'est pas un nombre valide.
pub fn parse_number_string(s: &str) -> Result<Vec<i32>, String> {
    s.trim()
        .split(',')
        .filter(|e| !e.is_empty())
        .map(|e| e.trim().parse::<i32>().map_err(|_| e.to_string()))
        .collect::<Result<Vec<i32>, String>>()
        .map_err(|e| format!("Erreur de parsing sur l'élément '{}'", e))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_string_with_positives_and_negatives() {
        assert_eq!(
            parse_number_string("1,2,-3,4, -5"),
            Ok(vec![1, 2, -3, 4, -5])
        );
    }

    #[test]
    fn test_string_with_whitespace() {
        assert_eq!(
            parse_number_string("  10, 20 ,-30  "),
            Ok(vec![10, 20, -30])
        );
    }

    #[test]
    fn test_string_with_invalid_element() {
        assert_eq!(
            parse_number_string("1,2,trois,4"),
            Err("Erreur de parsing sur l'élément 'trois'".to_string())
        );
    }

    #[test]
    fn test_string_with_invalid_element_at_end() {
        assert_eq!(
            parse_number_string("1,2,3,4,cinq"),
            Err("Erreur de parsing sur l'élément 'cinq'".to_string())
        );
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(parse_number_string(""), Ok(vec![]));
    }

    #[test]
    fn test_string_with_only_whitespace_and_commas() {
        assert_eq!(parse_number_string("  , ,   ,"), Ok(vec![]));
    }

    #[test]
    fn test_string_with_empty_segments() {
        assert_eq!(parse_number_string("1,,2,-3,"), Ok(vec![1, 2, -3]));
    }

    #[test]
    fn test_single_valid_number() {
        assert_eq!(parse_number_string("42"), Ok(vec![42]));
    }

    #[test]
    fn test_single_invalid_number() {
        assert_eq!(
            parse_number_string("quarante-deux"),
            Err("Erreur de parsing sur l'élément 'quarante-deux'".to_string())
        );
    }
}
