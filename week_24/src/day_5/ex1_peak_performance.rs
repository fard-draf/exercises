// 🎯 Objectif: 3 fonctions en < 5min chacune
// Réveil des patterns d'hier matin

// 1. Sum of squares
fn sum_squares(nums: &[i32]) -> i32 {
    // fold pattern simple
    nums.iter().fold(0, |mut acc, nbr| {
        acc += nbr.pow(2);
        acc
    })
}
//1min21

// 2. Count vowels
fn count_vowels(text: &str) -> usize {
    // fold sur chars
    let vowels = "aeiouAEIOU";
    text.chars().filter(|c| vowels.contains(*c)).count()
}
// 2min

// 3. Product of non-zero
fn product_non_zero(nums: &[i32]) -> i32 {
    // fold avec filter intégré
    nums.iter().filter(|e| **e != 0).fold(1, |mut acc, nbr| {
        acc *= *nbr;
        acc
    })
}
//1min

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_squares() {
        assert_eq!(sum_squares(&[1, 2, 3]), 14); // 1+4+9
        assert_eq!(sum_squares(&[]), 0);
    }

    #[test]
    fn test_count_vowels() {
        assert_eq!(count_vowels("hello world"), 3);
        assert_eq!(count_vowels("xyz"), 0);
    }

    #[test]
    fn test_product_non_zero() {
        assert_eq!(product_non_zero(&[1, 0, 2, 3, 0]), 6);
        assert_eq!(product_non_zero(&[0, 0]), 1);
    }
}
