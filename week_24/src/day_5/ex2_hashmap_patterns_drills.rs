use std::collections::HashMap;
// 🎯 Objectif: Automatiser HashMap + or_default()

// 3 exercices progressifs, 15min max chacun

// 1. Character frequency (5min target)
fn char_frequency(text: &str) -> HashMap<char, usize> {
    // Pattern: entry().or_default()
    text.chars()
        .fold(HashMap::with_capacity(text.len()), |mut acc, char| {
            acc.entry(char).and_modify(|e| *e += 1).or_insert(1);
            acc
        })
}

//6 min

// 2. Group by first letter (10min target)
fn group_by_first_letter(words: &[&str]) -> HashMap<char, Vec<String>> {
    // Pattern: entry().or_default().push()
    words.iter().fold(HashMap::new(), |mut acc, words| {
        let clean_w = words.trim().to_lowercase();
        let char = clean_w.chars().next();
        if let Some(char) = char {
            acc.entry(char).or_default().push(clean_w);
        }
        acc
    })
}
// 6 min

// 3. Word length histogram (15min target)
fn length_histogram(text: &str) -> HashMap<usize, usize> {
    // Combiner split_whitespace() + fold + HashMap
    text.split_whitespace()
        .fold(HashMap::new(), |mut acc, words| {
            acc.entry(words.len()).or_insert(words.len());
            acc
        })
}

// 3 min -> Pas de test et context restrein. Je supppose que c est ok

//15 min total

#[cfg(test)]
mod hashmap_tests {
    use super::*;

    #[test]
    fn test_char_frequency() {
        let freq = char_frequency("hello");
        assert_eq!(freq.get(&'l'), Some(&2));
        assert_eq!(freq.get(&'h'), Some(&1));
    }

    #[test]
    fn test_group_by_first() {
        let grouped = group_by_first_letter(&["apple", "ant", "bear", "bee"]);
        assert_eq!(grouped.get(&'a').unwrap().len(), 2);
        assert_eq!(grouped.get(&'b').unwrap(), &vec!["bear", "bee"]);
    }
}
