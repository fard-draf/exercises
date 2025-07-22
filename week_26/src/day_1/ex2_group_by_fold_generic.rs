pub fn group_by_key<K, V, F>(iter: impl Iterator<Item = V>, key_fn: F) -> Vec<(K, Vec<V>)>
where
    K: PartialEq + Clone,
    F: Fn(&V) -> K,
{
    let init_acc = (Vec::<(K, Vec<V>)>::new(), None::<(K, Vec<V>)>);
    let (mut final_group, last_group) = iter.fold(init_acc, |mut acc, item| {
        let key = key_fn(&item);

        if let Some((running_key, mut vec_val)) = acc.1.take() {
            if running_key == key {
                vec_val.push(item);
                acc.1 = Some((key, vec_val));
            } else {
                acc.0.push((running_key, vec_val));
                acc.1 = Some((key, vec![item]));
            }
        } else {
            acc.1 = Some((key, vec![item]));
        }

        acc
    });

    if let Some(value) = last_group {
        final_group.push(value);
    }

    final_group
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Clone)]
    struct City {
        name: String,
        country: String,
    }

    #[test]
    fn test_group_by_country() {
        let cities = vec![
            City {
                name: "Paris".to_string(),
                country: "France".to_string(),
            },
            City {
                name: "Lyon".to_string(),
                country: "France".to_string(),
            },
            City {
                name: "Berlin".to_string(),
                country: "Germany".to_string(),
            },
            City {
                name: "Quimper".to_string(),
                country: "France".to_string(),
            }, // Note: not sorted
        ];

        // Attention: ce pattern `group_by` ne regroupe que les éléments consécutifs.
        // Un véritable `group_by` (comme celui de itertools ou d'un HashMap)
        // regrouperait tous les français ensemble. C'est une distinction importante !
        let grouped = group_by_key(cities.into_iter(), |c| c.country.clone());

        assert_eq!(grouped.len(), 3);
        assert_eq!(grouped[0].0, "France");
        assert_eq!(grouped[0].1.len(), 2);
        assert_eq!(grouped[1].0, "Germany");
        assert_eq!(grouped[1].1.len(), 1);
        assert_eq!(grouped[2].0, "France");
        assert_eq!(grouped[2].1.len(), 1);
    }

    #[test]
    fn test_empty_iterator() {
        let empty_vec: Vec<City> = vec![];
        let grouped = group_by_key(empty_vec.into_iter(), |c| c.country.clone());
        assert!(grouped.is_empty());
    }

    #[test]
    fn test_single_element() {
        let cities = vec![City {
            name: "Tokyo".to_string(),
            country: "Japan".to_string(),
        }];
        let grouped = group_by_key(cities.into_iter(), |c| c.country.clone());
        assert_eq!(grouped.len(), 1);
        assert_eq!(grouped[0].0, "Japan");
        assert_eq!(grouped[0].1.len(), 1);
    }

    #[test]
    fn test_all_same_key() {
        let cities = vec![
            City {
                name: "Paris".to_string(),
                country: "France".to_string(),
            },
            City {
                name: "Lyon".to_string(),
                country: "France".to_string(),
            },
            City {
                name: "Marseille".to_string(),
                country: "France".to_string(),
            },
        ];
        let grouped = group_by_key(cities.into_iter(), |c| c.country.clone());
        assert_eq!(grouped.len(), 1);
        assert_eq!(grouped[0].0, "France");
        assert_eq!(grouped[0].1.len(), 3);
    }
}
