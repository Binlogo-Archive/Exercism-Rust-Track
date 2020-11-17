use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let chunks = input.chunks(worker_count);
    let mut handles = vec![];
    for chunk in chunks {
        let chunk = chunk
            .into_iter()
            .flat_map(|s| s.chars().filter(|c| c.is_alphabetic()))
            .flat_map(|c| c.to_lowercase())
            .collect();
        handles.push(thread::spawn(move || _frequency(chunk)));
    }

    let mut result = HashMap::new();
    for handle in handles {
        for (c, count) in handle.join().unwrap() {
            *result.entry(c).or_default() += count;
        }
    }

    return result;
}

fn _frequency(input: String) -> HashMap<char, usize> {
    input
        .chars()
        .fold(HashMap::new(), |mut r: HashMap<char, usize>, char| {
            *r.entry(char).or_default() += 1;
            return r;
        })
}
