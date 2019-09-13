use std::u32;
pub fn nth(n: u32) -> u32 {
    // unimplemented!("What is the 0-indexed {}th prime number?", n)
    (2..u32::MAX)
        .filter(|num| is_prime(*num))
        .nth(n as usize)
        .unwrap()
}

fn is_prime(n: u32) -> bool {
    if n <= 2 {
        return n == 2;
    }
    for i in 2..(n / 2 + 1) {
        if n % i == 0 {
            return false;
        }
    }
    true
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
