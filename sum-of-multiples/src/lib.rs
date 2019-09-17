pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit).filter(|n| is_multiple_of(factors, n)).sum()
}

fn is_multiple_of(factors: &[u32], n: &u32) -> bool {
    for factor in factors {
        if *factor == 0 {
            continue;
        }
        if n % factor == 0 {
            return true;
        }
    }
    false
}
