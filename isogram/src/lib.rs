use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut exists = HashSet::new();
    candidate
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .all(|c| exists.insert(c))
}
