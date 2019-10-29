use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut exists = HashSet::new();
    for c in candidate
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
    {
        if exists.contains(&c) {
            return false;
        } else {
            exists.insert(c);
        }
    }
    true
}
