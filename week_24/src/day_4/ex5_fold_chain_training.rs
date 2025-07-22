// 📋 CONTEXTE COMPLET:
// Utiliser fold multiple fois dans la même fonction
// But: Comprendre quand un fold suffit vs quand enchaîner

// 1. Double fold nécessaire
fn word_frequency_sorted(text: &str) -> Vec<(String, usize)> {
    // 1er fold: Compter fréquence des mots
    // 2ème fold: Trier par fréquence décroissante
    // Ignorer casse et ponctuation basique (. , ! ?)
    // Ex: "Hello world. Hello!" -> vec![("hello", 2), ("world", 1)]
    let clean_txt = text
        .to_lowercase()
        .chars()
        .filter(|e| !e.is_ascii_punctuation())
        .collect::<String>();

    let mut str = clean_txt.split_whitespace().collect::<Vec<&str>>();

    let vec = str
        .iter_mut()
        .fold((Vec::<(String, usize)>::new()), |mut acc, words| {
            if let Some(value) = acc.iter_mut().find(|e| e.0 == *words) {
                value.1 += 1;
            } else {
                acc.push((words.to_string(), 1));
            }

            acc
        });

    vec
}

// 30 minutes

// 2. Un seul fold suffit (piège!)
fn running_average(numbers: &[f64]) -> Vec<f64> {
    // Retourner les moyennes cumulatives
    // [1.0, 2.0, 3.0] -> [1.0, 1.5, 2.0]
    // NE PAS faire map().collect(), utiliser fold
    let (vec, old, count) = numbers.iter().fold(
        (Vec::new(), Vec::new(), 1.0),
        |(mut vec, mut old, mut count), nbr| {
            if vec.is_empty() {
                old.push(*nbr);
                vec.push(*nbr);
            } else {
                let sum = (old.iter().sum::<f64>() + nbr) / count;
                vec.push(sum);
                println!("old:{:?} nbr:{} count: {}", old, nbr, count);
                old.push(*nbr);
            }
            count += 1.0;
            (vec, old, count)
        },
    );

    vec
}

//30 minutes

// // 3. Fold avec look-back
// fn differences(numbers: &[i32]) -> Vec<i32> {
//     // Différences entre éléments consécutifs
//     // [1, 3, 6, 10] -> [2, 3, 4]
//     // Vide ou 1 élément -> vec![]
//     todo!()
// }

// // 4. Fold avec état complexe
// fn longest_increasing_sequence(numbers: &[i32]) -> Vec<i32> {
//     // Trouver la plus longue sous-séquence croissante CONTIGUË
//     // [1,2,3,1,2,3,4,5] -> [1,2,3,4,5]
//     // [5,4,3,2,1] -> [5] (un seul élément)
//     todo!()
// }

#[cfg(test)]
mod tests_chain {
    use super::*;

    #[test]
    fn test_word_freq() {
        let result = word_frequency_sorted("hello world hello");
        assert_eq!(
            result,
            vec![("hello".to_string(), 2), ("world".to_string(), 1)]
        );
    }

    #[test]
    fn test_running_avg() {
        assert_eq!(
            running_average(&[1.0, 2.0, 3.0, 4.0]),
            vec![1.0, 1.5, 2.0, 2.5]
        );
        assert_eq!(running_average(&[]), vec![]);
    }

    // #[test]
    // fn test_differences() {
    //     assert_eq!(differences(&[1, 3, 6, 10]), vec![2, 3, 4]);
    //     assert_eq!(differences(&[5]), vec![]);
    // }

    // #[test]
    // fn test_longest_seq() {
    //     assert_eq!(longest_increasing_sequence(&[1,2,3,1,2,3,4,5]), vec![1,2,3,4,5]);
    //     assert_eq!(longest_increasing_sequence(&[5,4,3,2,1]), vec![5]);
    // }
}
