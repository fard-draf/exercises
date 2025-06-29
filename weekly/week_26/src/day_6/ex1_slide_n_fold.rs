pub fn analyze_sliding_windows(data: &[u32], window_size: usize) -> Vec<(usize, u32, f64)> {
    if window_size == 0 || window_size > data.len() {
        return Vec::new();
    }

    let initial = (0u32, 0f64);

    data.windows(window_size).enumerate().scan(initial, |state, (index, window)| {

        if index == 0 {
            state.0 = window.iter().sum();
        } else {
            let elem_out = data[index - 1];
            let elem_in = window.last().unwrap();
            state.0 = state.0 - elem_out + elem_in;
        }

        let average = state.0 as f64 / window_size as f64;

        Some((index, state.0, average))
    }).collect()
    

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nominal_case() {
        let data = &[10, 20, 30, 40, 50, 60];
        let window_size = 3;
        let expected = vec![
            (0, 60, 20.0),
            (1, 90, 30.0),
            (2, 120, 40.0),
            (3, 150, 50.0),
        ];
        assert_eq!(analyze_sliding_windows(data, window_size), expected);
    }

    #[test]
    fn test_empty_data() {
        let data = &[];
        let window_size = 3;
        assert!(analyze_sliding_windows(data, window_size).is_empty());
    }

    #[test]
    fn test_window_size_zero() {
        let data = &[1, 2, 3];
        let window_size = 0;
        assert!(analyze_sliding_windows(data, window_size).is_empty());
    }

    #[test]
    fn test_window_size_one() {
        let data = &[10, 20, 30];
        let window_size = 1;
        let expected = vec![(0, 10, 10.0), (1, 20, 20.0), (2, 30, 30.0)];
        assert_eq!(analyze_sliding_windows(data, window_size), expected);
    }

    #[test]
    fn test_window_size_equals_data_len() {
        let data = &[10, 20, 30];
        let window_size = 3;
        let expected = vec![(0, 60, 20.0)];
        assert_eq!(analyze_sliding_windows(data, window_size), expected);
    }

    #[test]
    fn test_window_size_greater_than_data_len() {
        let data = &[10, 20, 30];
        let window_size = 5;
        assert!(analyze_sliding_windows(data, window_size).is_empty());
    }
}