fn two_sort(arr: &[&str]) -> String {

    let mut vec = arr.to_vec();
    vec.sort_by(|a, b| a.cmp(b));
    let first = vec[0];
    first.chars()
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .join("***")
    // first.chars().fold(Vec::<String>::new(), |mut acc, char| {
    //     acc.push(char.to_string());
    //     acc
    // }).join("***");

  
   
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test_cases() {
        assert_eq!(two_sort(&["bitcoin", "take", "over", "the", "world", "maybe", "who", "knows", "perhaps"]), "b***i***t***c***o***i***n");
        assert_eq!(two_sort(&["turns", "out", "random", "test", "cases", "are", "easier", "than", "writing", "out", "basic", "ones"]), "a***r***e");
    }
}