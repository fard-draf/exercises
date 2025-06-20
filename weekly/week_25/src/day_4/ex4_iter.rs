/// Partitionne une collection d'entiers en nombres pairs et impairs en une seule passe
/// en utilisant la méthode `fold`.
pub fn partition_with_fold<I>(iter: I) -> (Vec<i32>, Vec<i32>)
where
    I: IntoIterator<Item = i32>,
{
    iter.into_iter()
        .fold((Vec::new(), Vec::new()), |(mut vec1, mut vec2), val| {
            if val % 2 == 0 {
                vec1.push(val);
            } else {
                vec2.push(val);
            }
            (vec1, vec2)
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    // Helper pour comparer des vecs sans se soucier de l'ordre
    fn assert_vec_equiv(a: Vec<i32>, b: Vec<i32>) {
        let set_a: HashSet<_> = a.into_iter().collect();
        let set_b: HashSet<_> = b.into_iter().collect();
        assert_eq!(set_a, set_b);
    }

    #[test]
    fn test_partition_mixed_numbers() {
        let data = vec![1, 2, 3, 4, 5, 6, 0, -1, -2];
        let (evens, odds) = partition_with_fold(data);
        assert_vec_equiv(evens, vec![2, 4, 6, 0, -2]);
        assert_vec_equiv(odds, vec![1, 3, 5, -1]);
    }

    #[test]
    fn test_partition_empty_input() {
        let data: Vec<i32> = vec![];
        let (evens, odds) = partition_with_fold(data);
        assert!(evens.is_empty());
        assert!(odds.is_empty());
    }

    #[test]
    fn test_partition_only_evens() {
        let data = vec![2, 4, 8, -10];
        let (evens, odds) = partition_with_fold(data);
        assert_vec_equiv(evens, vec![2, 4, 8, -10]);
        assert!(odds.is_empty());
    }

    #[test]
    fn test_partition_only_odds() {
        let data = vec![1, 3, 9, -11];
        let (evens, odds) = partition_with_fold(data);
        assert_vec_equiv(odds, vec![1, 3, 9, -11]);
        assert!(evens.is_empty());
    }
}
