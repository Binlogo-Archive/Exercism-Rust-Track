/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    if isbn.is_empty() {
        return false;
    }
    let isbn = isbn.replace("-", "");
    if isbn.len() != 10 {
        return false;
    }
    let mut sum = 0;
    for (idx, c) in isbn.chars().enumerate() {
        let base = 10 - idx as u32;
        let acc = match c {
            'X' => {
                if idx == 9 {
                    Some(10 * base)
                } else {
                    None
                }
            }
            _ => c.to_digit(10).map(|v| v * base),
        };
        if let Some(value) = acc {
            sum += value;
        } else {
            return false;
        }
    }
    sum % 11 == 0
}

#[test]
#[ignore]
fn test_valid_check_digit_of_10() {
    assert!(is_valid_isbn("3-598-21507-X"));
}
