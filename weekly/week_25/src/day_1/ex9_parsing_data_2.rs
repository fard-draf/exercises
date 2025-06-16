// 🎯 [Closures & Iterators (try_fold)] - [Intermédiaire vs Intermédiaire] - [~45min]
//
// ⚓ MISSION :
// Valider et parser une chaîne de consignes de navigation. La chaîne contient une
// séquence de changements de cap numériques, séparés par des virgules.
// Votre fonction doit retourner la liste de tous les changements de cap si la
// séquence est valide, ou échouer proprement à la première valeur invalide.
// Cet exercice synthétise tout le contenu de la journée W25D1.
//
// 📋 SPÉCIFICATIONS :
// - Input: Une référence de chaîne de caractères (`&str`).
// - Output: Un `Result<Vec<i32>, String>`.
//   - `Ok(Vec<i32>)` si toutes les valeurs sont des entiers valides.
//   - `Err(String)` si une valeur n'est pas un entier valide.
// - Comportement: Le traitement doit s'arrêter immédiatement à la première
//   erreur de parsing et retourner une erreur descriptive.
//
// 📐 RÈGLES & CAS LIMITES :
// 1. Le parsing doit être strict : tout élément qui n'est pas un `i32` valide
//    doit provoquer une erreur.
// 2. La chaîne d'erreur doit être formatée ainsi : "Erreur de parsing sur l'élément: '[valeur erronée]'".
// 3. Une chaîne d'entrée vide est valide et doit produire un vecteur vide : `Ok(vec![])`.
// 4. Les espaces autour des nombres doivent être ignorés (ex: " 1 , -2 ").
//
// 🧪 EXEMPLES :
// parse_waypoints("10,-5,30, -15") -> Ok(vec![10, -5, 30, -15])
// parse_waypoints("") -> Ok(vec![])
// parse_waypoints("10,vingt,30") -> Err("Erreur de parsing sur l'élément: 'vingt'".to_string())

// CODE STARTER - Ne modifiez pas la structure
fn parse_waypoints(consignes: &str) -> Result<Vec<i32>, String> {
    if consignes.is_empty() {
        return Ok(Vec::new());
    }
    consignes
        .split(',')
        .try_fold(Vec::<i32>::new(), |mut acc, segment| {
            let data = segment
                .trim()
                .parse::<i32>()
                .map_err(|_| format!("Erreur de parsing sur l'élément: '{}'", segment))?;

            acc.push(data);

            Ok(acc)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cas_nominal() {
        assert_eq!(
            parse_waypoints("10,20,-5,0,100"),
            Ok(vec![10, 20, -5, 0, 100])
        );
    }

    #[test]
    fn test_avec_espaces() {
        assert_eq!(parse_waypoints(" 1 , -2,3 "), Ok(vec![1, -2, 3]));
    }

    #[test]
    fn test_cas_limite_vide() {
        assert_eq!(parse_waypoints(""), Ok(vec![]));
    }

    #[test]
    fn test_un_seul_element_valide() {
        assert_eq!(parse_waypoints("42"), Ok(vec![42]));
    }

    #[test]
    fn test_un_seul_element_invalide() {
        assert_eq!(
            parse_waypoints("quarante-deux"),
            Err("Erreur de parsing sur l'élément: 'quarante-deux'".to_string())
        );
    }

    #[test]
    fn test_erreur_au_milieu() {
        assert_eq!(
            parse_waypoints("10,20,pas_un_nombre,40"),
            Err("Erreur de parsing sur l'élément: 'pas_un_nombre'".to_string())
        );
    }

    #[test]
    fn test_erreur_a_la_fin() {
        assert_eq!(
            parse_waypoints("10,20,30,stop"),
            Err("Erreur de parsing sur l'élément: 'stop'".to_string())
        );
    }

    #[test]
    fn test_erreur_au_debut() {
        assert_eq!(
            parse_waypoints("erreur,1,2"),
            Err("Erreur de parsing sur l'élément: 'erreur'".to_string())
        );
    }
}

fn main() {
    // Zone de test rapide pour vérifier votre implémentation avant de lancer `cargo test`
    // let consignes1 = "10,-5,30, -15";
    // println!("Test 1: {:?} -> {:?}", consignes1, parse_waypoints(consignes1));
}
