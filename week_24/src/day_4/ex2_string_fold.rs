// 🎯 Challenge: String Fold Patterns
// 📊 Niveau: Easy-Medium
// ⏱️ Durée: 1h

use std::collections::HashMap;

pub fn word_lengths_fold(text: &str) -> Vec<usize> {
    // TODO: Utiliser fold pour collecter les longueurs de mots
    // Accumulateur: Vec<usize> en construction
    // Parser caractère par caractère
    let (mut sum, count) = text.chars().fold((vec![], 0), |mut acc, char| {
        if char.is_alphabetic() {
            acc.1 += 1;
        }
        if char.is_whitespace() && acc.1 != 0 {
            acc.0.push(acc.1);
            acc.1 = 0;
        }
        acc
    });
    if count != 0 {
        sum.push(count);
    }
    sum
}

// 7 min 25

pub fn capitalize_words_fold(text: &str) -> String {
    // TODO: Utiliser fold pour capitaliser chaque mot
    // Accumulateur: (String, bool) où bool = "prochain char est début de mot"
    // État: construction progressive de la string
    let result = text.chars().fold(
        (String::with_capacity(text.len()), true),
        |mut acc, char| match acc.1 {
            false => {
                acc.0.push(char);
                acc.1 = char.is_whitespace();
                acc
            }
            true => {
                if !char.is_whitespace() {
                    let char = char.to_uppercase().to_string();
                    acc.0.push_str(&char);
                    acc.1 = false;
                } else {
                    acc.0.push(char);
                }
                acc
            }
        },
    );

    result.0
}

// 10 min 34

pub fn group_by_length_fold(words: &[&str]) -> HashMap<usize, Vec<String>> {
    // TODO: Utiliser fold pour grouper les mots par longueur
    // Accumulateur: HashMap en construction
    // État: insertion dans le bon groupe
    words
        .iter()
        .fold(HashMap::<usize, Vec<String>>::new(), |mut acc, &string| {
            acc.entry(string.len())
                .or_default()
                .push(string.to_string());
            // .and_modify(|e| e.push(string.to_string()));
            println!("{:?}", acc);
            acc
        })
}
//13 min -> decouverte de or_default()

pub fn encode_rle_fold(text: &str) -> String {
    // TODO: Utiliser fold pour encoder en RLE
    // Accumulateur: (String result, Option<(char, count)>)
    // État: accumulation du compte ou flush + nouveau char
    let mut result = text.chars().fold(
        (String::new(), None::<(char, u32)>),
        |(mut s, mut acc), char| {
            match acc {
                None => {
                    println!("intiate");
                    acc = Some((char, 1))
                }
                Some((val, mut count)) if val == char => {
                    count += 1;
                    println!("{count}");
                    acc = Some((val, count))
                }
                Some((val, count)) if val != char => {
                    s.push_str(&count.to_string());
                    s.push(val);
                    let val = char;
                    println!("Should change {:?}", val);
                    acc = Some((val, 1))
                }
                _ => acc = None,
            };

            println!("{:?}", acc);
            (s, acc)
        },
    );

    if let Some((char, count)) = result.1 {
        result.0.push_str(&count.to_string());
        result.0.push(char);
    }

    result.0
}

// 25 minutes -> concept RLE nouveau, je suis parti a l envers au debut puis j ai solutionne le pb par moi meme ! Bonne progression.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_lengths_fold() {
        assert_eq!(word_lengths_fold("hello world rust"), vec![5, 5, 4]);
        assert_eq!(word_lengths_fold("  a  bb   ccc  "), vec![1, 2, 3]);
        assert_eq!(word_lengths_fold(""), vec![]);
        assert_eq!(word_lengths_fold("   "), vec![]);
        assert_eq!(word_lengths_fold("single"), vec![6]);
    }

    #[test]
    fn test_capitalize_words_fold() {
        assert_eq!(capitalize_words_fold("hello world"), "Hello World");
        assert_eq!(capitalize_words_fold("rust is awesome"), "Rust Is Awesome");
        assert_eq!(capitalize_words_fold(""), "");
        assert_eq!(capitalize_words_fold("a"), "A");
        assert_eq!(capitalize_words_fold("  spaced  out  "), "  Spaced  Out  ");
    }

    #[test]
    fn test_group_by_length_fold() {
        let words = vec!["hi", "bye", "rust", "code", "a"];
        let result = group_by_length_fold(&words);

        assert_eq!(result.get(&1), Some(&vec!["a".to_string()]));
        assert_eq!(result.get(&2), Some(&vec!["hi".to_string()]));
        assert_eq!(result.get(&3), Some(&vec!["bye".to_string()]));
        assert_eq!(
            result.get(&4),
            Some(&vec!["rust".to_string(), "code".to_string()])
        );
        assert_eq!(result.get(&5), None);

        let empty: Vec<&str> = vec![];
        assert_eq!(group_by_length_fold(&empty), HashMap::new());
    }

    #[test]
    fn test_encode_rle_fold() {
        assert_eq!(encode_rle_fold("aaabbc"), "3a2b1c");
        assert_eq!(encode_rle_fold("a"), "1a");
        assert_eq!(encode_rle_fold(""), "");
        assert_eq!(encode_rle_fold("aabbccddee"), "2a2b2c2d2e");
        assert_eq!(encode_rle_fold("abcdef"), "1a1b1c1d1e1f");
    }
}
