// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let words_count_map = magazine.into_iter().fold(HashMap::new(), |mut map, word| {
        *map.entry(word).or_insert(1) += 1;
        map
    });
    let node_count_map = note.into_iter().fold(HashMap::new(), |mut map, word| {
        *map.entry(word).or_insert(1) += 1;
        map
    });
    let result = node_count_map.into_iter().all(|(word, count)| {
        if words_count_map.contains_key(word) {
            return words_count_map[word] >= count;
        } else {
            return false;
        }
    });
    return result;
}
