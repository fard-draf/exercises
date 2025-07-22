fn fibonacci(n: usize) -> Vec<u32> {
    if n == 0 {
        return vec![];
    }

    if n == 1 {
        return vec![0];
    }

    if n == 2 {
        return vec![0, 1];
    }

    let range_iter = 0..=n;
    let result = range_iter.scan((0, 1), |state, _x| {
        let (current, next) = *state;
        let result = Some(current);
        *state = (next, current + next);

        result
    });

    result.collect::<Vec<u32>>()
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_fibo_special_cases() {
        assert_eq!(fibonacci(10), vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55])
    }
}
