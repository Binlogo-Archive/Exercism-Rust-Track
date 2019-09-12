use std::u32;
pub fn nth(n: u32) -> u32 {
    // unimplemented!("What is the 0-indexed {}th prime number?", n)
    let mut i = 0;
    let mut candidate = 2;

    while i < n {
        candidate += 1;
        if is_prime(candidate) {
            i += 1;
        }
    }
    candidate
}

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if n % 2 == 0 {
        return n == 2;
    }
    if n % 3 == 0 {
        return n == 3;
    }
    for i in 5..n {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

#[test]
fn test_is_prime_should_true() {
    let primes = vec![2, 3, 5, 7, 11, 13];
    for n in primes {
        assert_eq!(is_prime(n), true);
    }
}

#[test]
fn test_is_prime_should_false() {
    let primes = vec![1, 4, 6, 8, 10, 12];
    for n in primes {
        assert_eq!(is_prime(n), false);
    }
}
