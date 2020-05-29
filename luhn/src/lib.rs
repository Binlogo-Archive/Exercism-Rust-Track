/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    code.chars()
        .filter(|c| !c.is_whitespace())
        .rev()
        .try_fold((0, 0), |(i, checksum), c| {
            c.to_digit(10)
                .map(|d| if i % 2 == 0 { d } else { d * 2 })
                .map(|d| checksum + (if d > 9 { d - 9 } else { d }))
                .map(|d| (i + 1, d))
        })
        .map_or(false, |(len, checksum)| len > 1 && checksum % 10 == 0)
}
