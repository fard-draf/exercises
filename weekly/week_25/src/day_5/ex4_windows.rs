fn find_max_consecutive_difference(numbers: &[i32]) -> Option<i32> {
    if numbers.len() < 2 {
        return None;
    }

    numbers
        .windows(2)
        .map(|e| {
            let diff = e[0] - e[1];
            diff.abs()
        })
        .max()
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn test_windows() {
        // Objectif final d'utilisation
        let data = &[10, 15, 12, 18, 11];
        // Ecarts: |10-15|=5, |15-12|=3, |12-18|=6, |18-11|=7
        let max_diff = find_max_consecutive_difference(data);
        assert_eq!(max_diff, Some(7));
    }
}
