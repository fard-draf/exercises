// 🎯 Challenge: Calculateur de Score de Risque
// 📊 Niveau: Hard
// ⏱️ Durée: 1h 30min
//
// 📋 MISSION COMPLETE:
// Tu analyses des logs systèmes pour calculer un "score de risque" total.
// Chaque ligne de log a un format strict: "LEVEL:CODE:MESSAGE".
// - `LEVEL` peut être "INFO", "WARN", ou "ERROR".
// - `CODE` est un multiplicateur de risque (un nombre entier non signé).
// Le score d'une ligne est `valeur_du_level * code`. Le score de risque total est la somme des scores de toutes les lignes.
// Ta mission est de calculer ce score total. Si UNE SEULE ligne est malformée,
// le calcul entier doit s'arrêter et retourner une erreur.
//
// 📥 ENTRÉES:
// - `logs`: Un slice de chaînes de caractères (`&[&str]`).
//
// 📤 SORTIES:
// - `Success`: `Ok(u64)`, le score de risque total.
// - `Erreur`: `Err(LogError)`, décrivant la première erreur rencontrée.
//
// 📏 RÈGLES MÉTIER:
// 1. L'accumulateur de `try_fold` sera le score total (`u64`), initialisé à `0`.
// 2. Pour chaque ligne, tu dois la parser :
//    a. La diviser en trois parties sur le délimiteur `:`. Si le nombre de parties n'est pas 3, retourner `Err(LogError::Format)`.
//    b. Parser la première partie (`LEVEL`). Si le niveau est inconnu, retourner `Err(LogError::UnknownLevel)`.
//    c. Parser la seconde partie (`CODE`) en `u64`. Si ce n'est pas un nombre valide, retourner `Err(LogError::InvalidCode)`.
// 3. Les valeurs de base des niveaux sont : `ERROR` = 10, `WARN` = 5, `INFO` = 1.
// 4. Le score de la ligne est `valeur_du_level * code`.
// 5. La closure de `try_fold` doit retourner `Ok(score_accumulé + score_de_la_ligne)`.
//
// 💡 INDICE:
// Crée des petites fonctions helper pour le parsing. Par exemple, une fonction `parse_level(s: &str) -> Result<u64, LogError>`
// et utilise l'opérateur `?` à l'intérieur de ta closure `try_fold` pour chainer les opérations faillibles.

#[derive(Debug, PartialEq)]
pub enum LogError<'a> {
    Format(&'a str),
    UnknownLevel(&'a str),
    InvalidCode(&'a str),
}

// NOTE: Tu peux créer tes propres fonctions helper si tu le souhaites.

/// Calcule le score de risque total à partir d'un slice de logs.
/// Échoue à la première ligne de log malformée.
pub fn calculate_risk_score<'a>(logs: &'a [&str]) -> Result<u64, LogError<'a>> {
    logs.iter().try_fold(0, |mut acc, &log| {
        let mut parts = log.split(':');

        let level_part = parts.next().ok_or(LogError::Format(log))?;
        let code_str = parts.next().ok_or(LogError::Format(log))?;
        let _message_str = parts.next().ok_or(LogError::Format(log))?;

        if parts.next().is_some() {
            return Err(LogError::Format(log));
        }

        let level_value = match level_part {
            "INFO" => 1,
            "WARN" => 5,
            "ERROR" => 10,
            _ => return Err(LogError::UnknownLevel(log)),
        };

        let code_value = code_str
            .parse::<u64>()
            .map_err(|_| LogError::InvalidCode(log))?;

        Ok(acc + level_value * code_value)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_successful_score_calculation() {
        let logs = &[
            "INFO:10:User logged in",         // score = 1 * 10 = 10
            "WARN:50:Disk space low",         // score = 5 * 50 = 250
            "ERROR:300:DB connection failed", // score = 10 * 300 = 3000
        ];
        // Total = 10 + 250 + 3000 = 3260
        assert_eq!(calculate_risk_score(logs), Ok(3260));
    }

    #[test]
    fn test_fail_on_format_error() {
        let logs = &["INFO:10:OK", "WARN:50", "ERROR:300:FAIL"];
        assert_eq!(calculate_risk_score(logs), Err(LogError::Format("WARN:50")));
    }

    #[test]
    fn test_fail_on_unknown_level() {
        let logs = &["INFO:10:OK", "CRITICAL:999:System halted"];
        assert_eq!(
            calculate_risk_score(logs),
            Err(LogError::UnknownLevel("CRITICAL:999:System halted"))
        );
    }

    #[test]
    fn test_fail_on_invalid_code() {
        let logs = &["INFO:10:OK", "WARN:50A:Failed conversion"];
        assert_eq!(
            calculate_risk_score(logs),
            Err(LogError::InvalidCode("WARN:50A:Failed conversion"))
        );
    }

    #[test]
    fn test_empty_logs_return_zero() {
        let logs = &[];
        assert_eq!(calculate_risk_score(logs), Ok(0));
    }

    #[test]
    fn test_large_numbers() {
        let logs = &["ERROR:1000000:High traffic"]; // 10 * 1_000_000
        assert_eq!(calculate_risk_score(logs), Ok(10_000_000));
    }
}
