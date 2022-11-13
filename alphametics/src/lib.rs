use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    println!("input: {}", input);

    let firsts: HashSet<char> = input
        .split(&['+', '='])
        .filter_map(|s| s.trim().chars().next())
        .collect();

    println!("firsts: {:?}", firsts);

    let (letters, factors) = parse(input);

    println!("letters: {:?}, factros: {:?}", letters, factors);

    for perm in (0..=9).permutations(letters.len()) {
        let sum = cal_perm_sum(&perm, &factors);
        if sum == 0
            && !perm
                .iter()
                .enumerate()
                .any(|(i, v)| *v == 0 && firsts.contains(letters.get(i).unwrap()))
        {
            return Some(HashMap::from_iter(
                perm.iter()
                    .enumerate()
                    .map(|(i, v)| (*letters.get(i).unwrap(), *v as u8)),
            ));
        }
    }

    None
}

fn cal_perm_sum(perm: &[i64], factors: &[i64]) -> i64 {
    perm.iter().enumerate().map(|(i, &n)| n * factors[i]).sum()
}

fn parse(input: &str) -> (Vec<char>, Vec<i64>) /*(letters, factors)*/ {
    let mut factors = HashMap::new();
    let mut sign = -1;
    let mut pos = 0;

    for c in input.chars().filter(|c| !c.is_whitespace()).rev() {
        match c {
            '=' => {
                sign = 1;
                pos = 0;
            }
            '+' => {
                pos = 0;
            }
            _ => {
                let factor = factors.entry(c).or_insert(0);
                *factor += sign * 10i64.pow(pos as u32);
                pos += 1;
            }
        }
    }
    factors.into_iter().sorted_by_key(|(_, v)| -v.abs()).unzip()
}

