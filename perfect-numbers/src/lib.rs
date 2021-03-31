#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num < 1 {
        return None;
    }
    if num == 1 {
        return Some(Classification::Deficient);
    }
    let sum = factions(num).into_iter().fold(0, |acc, i| acc + i);
    if sum == num {
        Some(Classification::Perfect)
    } else if sum > num {
        Some(Classification::Abundant)
    } else {
        Some(Classification::Deficient)
    }
}

fn factions(num: u64) -> Vec<u64> {
    (1..)
        .take_while(|i| i * i <= num)
        .filter(|i| num % i == 0)
        .flat_map(|i| {
            if i == 1 || i == num / i {
                vec![i]
            } else {
                vec![i, num / i]
            }
        })
        .collect()
}

#[test]
fn test_factions() {
    println!("{:?}", factions(6));
    assert!(factions(6) == vec![1, 2, 3])
}
