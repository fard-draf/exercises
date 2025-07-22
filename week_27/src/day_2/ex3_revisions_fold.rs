pub fn max_sliding_window_sum(slice: &[i32], window_size: usize) -> Option<i32> {
    if window_size == 0 || slice.len() < window_size {
        return None;
    }
    

   slice.windows(window_size).map(|window| {
        window.iter().sum::<i32>()
    }).max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_case() {
        let data = &[1, 2, 3, 4, 5, 6];
        assert_eq!(max_sliding_window_sum(data, 3), Some(15));
    }

    #[test]
    fn test_with_negative_numbers() {
        let data = &[10, -2, 5, 8, -4, 7];
        assert_eq!(max_sliding_window_sum(data, 2), Some(13));
    }

    #[test]
    fn test_all_negative() {
        let data = &[-1, -2, -3, -4];
        assert_eq!(max_sliding_window_sum(data, 2), Some(-3));
    }

    #[test]
    fn test_window_too_large() {
        let data = &[1, 2, 3];
        assert_eq!(max_sliding_window_sum(data, 5), None);
    }

    #[test]
    fn test_empty_slice() {
        let data = &[];
        assert_eq!(max_sliding_window_sum(data, 3), None);
    }

    #[test]
    fn test_zero_window_size() {
        let data = &[1, 2, 3, 4, 5];
        assert_eq!(max_sliding_window_sum(data, 0), None);
    }
}