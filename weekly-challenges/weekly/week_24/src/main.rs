// 🎯 Challenge: Text Analysis with State Tracking
// 📊 Niveau: Easy-Medium (Consolidation)
// ⏱️ Durée: 45min

pub fn alternating_caps_fold(text: &str) -> String {
    // Alterner majuscule/minuscule sur les LETTRES uniquement
    // "hello world!" → "HeLlO wOrLd!"
    // Les espaces/ponctuation ne comptent pas dans l'alternance
    // Acc: (String, bool) où bool = prochaine lettre en majuscule
    let (result, _) = text.chars().fold(
        (String::with_capacity(text.len()), false),
        |(mut acc, mut next), char| {
            match ((char.is_whitespace() || char.is_numeric()), next) {
                (false, false) => (acc.push(char.to_ascii_uppercase()), next = true),
                (false, true) => (acc.push(char.to_ascii_lowercase()), next = false),
                (true, false) => (acc.push(char), next = false),
                (true, true) => (acc.push(char), next = true),
            };

            (acc, next)
        },
    );
    result
}

//34min sans aide exterieure

pub fn find_longest_word_fold(text: &str) -> String {
    // Trouver le mot le plus long (séparé par espaces)
    // En cas d'égalité, garder le premier
    // "" si texte vide ou que des espaces
    // Acc: (current_word, longest_word)

    let (longest, _, _) = text.chars().fold(
        ("".to_string(), "".to_string(), "".to_string()),
        |(mut current, longest, mut acc), char| {
            if !char.is_whitespace() {
                acc.push(char);
                acc.clone()
            } else {
                current.drain(..);
                current.push_str(&acc);
                acc.drain(..);
                current.clone()
            };

            let longest = match (current.len(), longest.len()) {
                (curlen, longlen) if curlen > longlen => current.clone(),
                (curlen, longlen) if curlen == longlen => longest,
                _ => longest,
            };

            (longest, current, acc)
        },
    );
    longest
}

// 1h // bloque sur le dernier longest qui revient tjrs, j arrive pas a garder le premier.

pub fn bracket_validator_fold(text: &str) -> bool {
    // Vérifier que les parenthèses sont équilibrées
    // "(hello (world))" → true
    // "((hello)" → false
    // ")" → false (fermeture sans ouverture)
    // Acc: compteur de parenthèses ouvertes (ou Option<i32> pour gérer erreur)
    let (result, _) = text
        .chars()
        .fold((0, "".to_string()), |(count, mut acc), char| match char {
            '(' => {
                println!("( +1");
                acc.push(char);
                (count + 1, acc.clone())
            }

            ')' => {
                if acc.contains('(') {
                    println!(") -1");
                    acc.push(char);
                    (count - 1, acc.clone())
                } else {
                    (count - 2, acc.clone())
                }
            }

            _ => (count, acc.clone()),
        });

    result == 0
}

//40 minutes -> bloque sur le )( mais trouve solution sans aide ext

pub fn extract_numbers_fold(text: &str) -> Vec<i32> {
    // Extraire tous les nombres du texte
    // "abc123def45ghi6" → vec![123, 45, 6]
    // Nombres négatifs supportés : "x-42y" → vec![-42]
    // Mais "-" seul n'est pas un nombre
    // Acc: (Vec<i32>, Option<String>) où String = nombre en construction

    let (result,_) = text.chars()
        .fold((Vec::<i32>::new(), None::<String>), |(mut acc, mut constr), char | {
            if char.is_ascii_digit() { 
                match constr {
                    Some(ref mut num_str) => num_str.push(char),
                    None => constr = Some(char.to_string())
                }
            }  else {
                if let Some(num_str) = constr.take() {
                    if let Ok(num) = num_str.parse()> {
                        acc.push(char);
                    }
                }
                constr = None;
            }

            (acc, constr)

        });

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alternating_caps() {
        assert_eq!(alternating_caps_fold("hello world"), "HeLlO wOrLd");
        assert_eq!(alternating_caps_fold("a b c"), "A b C");
        assert_eq!(alternating_caps_fold("123abc"), "123AbC");
        assert_eq!(alternating_caps_fold(""), "");
        assert_eq!(alternating_caps_fold("   "), "   ");
    }

    #[test]
    fn test_find_longest_word() {
        assert_eq!(find_longest_word_fold("the quick brown fox"), "quick");
        assert_eq!(find_longest_word_fold("a bb ccc dd e"), "ccc");
        assert_eq!(find_longest_word_fold("equal size"), "equal"); // Premier en cas d'égalité
        assert_eq!(find_longest_word_fold(""), "");
        assert_eq!(find_longest_word_fold("   "), "");
        assert_eq!(find_longest_word_fold("single"), "single");
    }

    #[test]
    fn test_bracket_validator() {
        assert!(bracket_validator_fold("(hello)"));
        assert!(bracket_validator_fold("(a(b)c)"));
        assert!(bracket_validator_fold("()()()"));
        assert!(bracket_validator_fold("no brackets here"));
        assert!(bracket_validator_fold(""));

        assert!(!bracket_validator_fold("(unclosed"));
        assert!(!bracket_validator_fold("unopened)"));
        assert!(!bracket_validator_fold(")("));
        assert!(!bracket_validator_fold("(a(b)"));
    }

    #[test]
    fn test_extract_numbers() {
        assert_eq!(extract_numbers_fold("abc123def45"), vec![123, 45]);
        assert_eq!(extract_numbers_fold("no numbers here"), vec![]);
        assert_eq!(extract_numbers_fold("-42 and 17"), vec![-42, 17]);
        assert_eq!(extract_numbers_fold("--5-"), vec![-5]); // Seul -5 est valide
        assert_eq!(extract_numbers_fold("0001"), vec![1]); // Leading zeros ok
        assert_eq!(extract_numbers_fold("a0b0c0"), vec![0, 0, 0]);
    }
}
