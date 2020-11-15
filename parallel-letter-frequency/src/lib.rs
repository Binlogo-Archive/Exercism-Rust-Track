use rayon::prelude::*;
use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    input
        .par_chunks(worker_count)
        .map(|chunk| _frequency(chunk))
        .reduce(
            || HashMap::new(),
            |mut acc, map| {
                map.into_iter()
                    .for_each(|(c, count)| *acc.entry(c).or_default() += count);
                acc
            },
        )
}

fn _frequency(input: &[&str]) -> HashMap<char, usize> {
    input
        .into_iter()
        .flat_map(|s| s.chars().filter(|c| c.is_alphabetic()))
        .flat_map(|c| c.to_lowercase())
        .fold(HashMap::new(), |mut r: HashMap<char, usize>, char| {
            *r.entry(char).or_default() += 1;
            return r;
        })
}
