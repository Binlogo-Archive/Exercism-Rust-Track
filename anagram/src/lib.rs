use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();
    let word_hash_map = hash_map_for(&word);
    possible_anagrams
        .iter()
        .filter(|candidate| {
            if candidate.len() != word.len() {
                return false;
            }
            let candidate = candidate.to_lowercase();
            candidate != word && hash_map_for(&candidate) == word_hash_map
        })
        .map(|x| *x)
        .collect::<HashSet<&str>>()
}

fn hash_map_for(word: &str) -> HashMap<char, u32> {
    word.chars().fold(HashMap::new(), |mut res, c| {
        *res.entry(c).or_insert(0) += 1;
        res
    })
}
