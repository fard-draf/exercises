use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct ParseError {
    pub faulty_line: String,
    pub partial_result: HashMap<String, i32>,
}

/// Agrège les données depuis un itérateur de chaînes formatées "clé:valeur".
/// S'arrête à la première ligne malformée et retourne une erreur contenant
/// le travail partiel.
pub fn aggregate_data<'a>(
    mut iter: impl Iterator<Item = &'a str>,
) -> Result<HashMap<String, i32>, ParseError> {
    let initial_state = HashMap::new();

    // TODO: Utilise try_fold ici.
    // La closure prendra l'accumulateur (HashMap) et l'item (&str).
    // Elle doit retourner un Result<HashMap<_,_>, ParseError>.
    // En cas de succès, le nouveau HashMap mis à jour est dans le Ok.
    // En cas d'échec, le HashMap *avant* la modification est dans le Err.
    iter.try_fold(initial_state, |mut acc, line| {
        if !line.contains(':') {
            return Err(ParseError {
                faulty_line: line.to_string(),
                partial_result: acc.clone(),
            });
        }
        if let Some((key, value)) = line.split_once(':') {
            let value = value.parse::<i32>().map_err(|_| ParseError {
                faulty_line: line.to_string(),
                partial_result: acc.clone(),
            })?;

            match key.trim().to_lowercase().as_str() {
                "power" | "level" => {
                    *acc.entry(key.to_string()).or_insert(0) += value;
                    Ok(acc)
                }
                _ => {
                    return Err(ParseError {
                        faulty_line: line.to_string(),
                        partial_result: acc.clone(),
                    });
                }
            }
        } else {
            Err(ParseError {
                faulty_line: line.to_string(),
                partial_result: acc.clone(),
            })
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_successful_aggregation() {
        let data = vec!["level:10", "power:5", "level:5", "power:-2"];
        let result = aggregate_data(data.into_iter()).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(*result.get("level").unwrap(), 15);
        assert_eq!(*result.get("power").unwrap(), 3);
    }

    #[test]
    fn test_error_no_delimiter() {
        // La ligne "power_5" est fautive. Le traitement s'arrête.
        // La map partielle ne doit contenir que "level:10".
        let data = vec!["level:10", "power_5", "level:5"];
        let err = aggregate_data(data.into_iter()).unwrap_err();

        assert_eq!(err.faulty_line, "power_5");
        assert_eq!(err.partial_result.len(), 1);
        assert_eq!(*err.partial_result.get("level").unwrap(), 10);
    }

    #[test]
    fn test_error_not_a_number() {
        // La ligne "power:abc" est fautive. Le traitement s'arrête.
        // La map partielle doit contenir "level:10" et "power:20" qui étaient valides.
        let data = vec!["level:10", "power:20", "power:abc", "level:5"];
        let err = aggregate_data(data.into_iter()).unwrap_err();

        assert_eq!(err.faulty_line, "power:abc");
        assert_eq!(err.partial_result.len(), 2);
        assert_eq!(*err.partial_result.get("level").unwrap(), 10);
        assert_eq!(*err.partial_result.get("power").unwrap(), 20);
    }

    #[test]
    fn test_empty_input() {
        let data: Vec<&str> = vec![];
        let result = aggregate_data(data.into_iter()).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_error_on_first_line() {
        let data = vec!["level-10", "level:10"];
        let err = aggregate_data(data.into_iter()).unwrap_err();
        assert_eq!(err.faulty_line, "level-10");
        assert!(err.partial_result.is_empty());
    }
}
