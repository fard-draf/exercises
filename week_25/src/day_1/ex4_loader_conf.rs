// 🎯 Challenge: Chargement de Configuration
// 📊 Niveau: Medium
// ⏱️ Durée: 1h
//
// 📋 MISSION COMPLETE:
// Tu dois implémenter un chargeur de configuration simple. La fonction prend un slice
// de chaînes de caractères, où chaque chaîne représente une ligne de configuration au
// format "CLE=VALEUR". Ta mission est de parser ces lignes et de les agréger dans une
// `HashMap`. Si une seule ligne n'est pas dans le bon format, le chargement complet
// doit échouer immédiatement. `try_fold` est l'outil idéal pour cette tâche.
//
// 📥 ENTRÉES:
// - `lines`: Un slice de chaînes de caractères (`&[&str]`).
//
// 📤 SORTIES:
// - `Success`: `Ok(HashMap<String, String>)`, contenant les paires clé/valeur.
// - `Erreur`: `Err(ParseError)`, si une ligne est malformée.
//
// 📏 RÈGLES MÉTIER:
// 1. L'accumulateur pour `try_fold` sera la `HashMap` en cours de construction.
// 2. La valeur initiale de l'accumulateur est une `HashMap` vide.
// 3. Pour chaque ligne, tu dois trouver la position du premier caractère `=`.
// 4. Si une ligne ne contient pas de `=`, la fonction doit immédiatement retourner `Err(ParseError::MissingSeparator)`.
// 5. La clé est la partie avant le premier `=`, et la valeur est toute la partie après.
//    Exemple: "URL=http://example.com?a=1" -> Clé: "URL", Valeur: "http://example.com?a=1".
// 6. Si une clé est dupliquée, la dernière valeur lue doit écraser la précédente (comportement par défaut de `HashMap::insert`).
// 7. Un slice d'entrée vide doit résulter en un `Ok(HashMap::new())`.
//
// 💡 INDICE:
// La closure de `try_fold` prendra une `HashMap` (l'accumulateur) et une ligne de config.
// Elle devra retourner un `Result<HashMap, ParseError>`. Tu devras rendre l'accumulateur
// mutable (`mut acc`) pour pouvoir y insérer des éléments avant de le retourner dans un `Ok`.

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum ParseError<'a> {
    MissingSeparator(&'a str),
}

/// Parse un slice de lignes de configuration en une HashMap.
/// Échoue à la première ligne malformée.
pub fn load_config<'a>(lines: &'a [&'a str]) -> Result<HashMap<String, String>, ParseError<'a>> {
    lines.iter().try_fold(HashMap::new(), |mut acc, &line| {
        if let Some(value) = line.split_once('=') {
            acc.insert(value.0.to_string(), value.1.to_string());
            Ok(acc)
        } else {
            Err(ParseError::MissingSeparator(line))
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_config_loading() {
        let lines = &["HOST=localhost", "PORT=8080", "USER=admin"];
        let mut expected = HashMap::new();
        expected.insert("HOST".to_string(), "localhost".to_string());
        expected.insert("PORT".to_string(), "8080".to_string());
        expected.insert("USER".to_string(), "admin".to_string());

        assert_eq!(load_config(lines), Ok(expected));
    }

    #[test]
    fn test_loading_fails_on_missing_separator() {
        let lines = &["HOST=localhost", "PORT:8080", "USER=admin"];
        assert_eq!(
            load_config(lines),
            Err(ParseError::MissingSeparator("PORT:8080"))
        );
    }

    #[test]
    fn test_empty_input_returns_empty_map() {
        let lines = &[];
        assert_eq!(load_config(lines), Ok(HashMap::new()));
    }

    #[test]
    fn test_last_value_wins_for_duplicate_keys() {
        let lines = &["MODE=test", "MODE=prod"];
        let mut expected = HashMap::new();
        expected.insert("MODE".to_string(), "prod".to_string());
        assert_eq!(load_config(lines), Ok(expected));
    }

    #[test]
    fn test_value_can_contain_equals_sign() {
        let lines = &["URL=https://myapi.com/query?key=value"];
        let mut expected = HashMap::new();
        expected.insert(
            "URL".to_string(),
            "https://myapi.com/query?key=value".to_string(),
        );
        assert_eq!(load_config(lines), Ok(expected));
    }

    #[test]
    fn test_empty_key_is_valid() {
        let lines = &["=myvalue"];
        let mut expected = HashMap::new();
        expected.insert("".to_string(), "myvalue".to_string());
        assert_eq!(load_config(lines), Ok(expected));
    }

    #[test]
    fn test_empty_value_is_valid() {
        let lines = &["MY_KEY="];
        let mut expected = HashMap::new();
        expected.insert("MY_KEY".to_string(), "".to_string());
        assert_eq!(load_config(lines), Ok(expected));
    }
}
