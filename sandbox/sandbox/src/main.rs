fn is_prime(x: i64) -> bool {
    if x.is_negative() {
        return false;
    }
        if (0..2).contains(&x) {
        return false;
    }


    // factorial calcul ((a-1)!)
    let mut factorial = 1;
    for i in 1..x{
        factorial = (factorial * i) % x;
    }
    // Wilson Theorem: p is prime nbr ioi ((p-1)! + 1) % p == 0;
    if ((factorial + 1) % x) == 0 {
        true
    } else {
        false
    }
} 

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn basic_tests() {
        assert!(!is_prime(0), "0 is not prime");
        assert!(!is_prime(1), "1 is not prime");
        assert!(is_prime(2), "2 is prime");
        assert!(is_prime(73), "73 is prime");
        assert!(!is_prime(75), "75 is not prime");
        assert!(!is_prime(-1), "-1 is not prime");
    }

    #[test]
    fn prime_tests() {
        assert!(is_prime(3), "3 is prime");
        assert!(is_prime(5), "5 is prime");
        assert!(is_prime(7), "7 is prime");
        assert!(is_prime(41), "41 is prime");
        assert!(is_prime(5099), "5099 is prime");
    }

    #[test]
    fn not_prime_tests() {
        assert!(!is_prime(4), "4 is not prime");
        assert!(!is_prime(6), "6 is not prime");
        assert!(!is_prime(8), "8 is not prime");
        assert!(!is_prime(9), "9 is not prime");
        assert!(!is_prime(45), "45 is not prime");
        assert!(!is_prime(-5), "-5 is not prime");
        assert!(!is_prime(-8), "-8 is not prime");
        assert!(!is_prime(-41), "-41 is not prime");
    }
}
