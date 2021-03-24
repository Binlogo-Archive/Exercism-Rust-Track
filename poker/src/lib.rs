/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
mod poker;
use poker::*;

// - Ranking a list of poker hands can be considered a sorting problem.
// - Rust provides the sort method for Vec<T> where T: Ord.
// - Ord types form a total order: exactly one of a < b, a == b, or a > b must be true.
// - Poker hands do not conform to a total order: it is possible for two hands to be non-equal but have equal sort order. Example: "3S 4S 5D 6H JH", "3H 4H 5C 6C JD".
// - Rust provides the PartialOrd trait to handle the case of sortable things which do not have a total order. However, it doesn't provide a standard sort method for Vec<T> where T: PartialOrd. The standard idiom to sort a vector in this case is your_vec.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::{Less|Equal|Greater}));, depending on your needs.
// - You might consider implementing a type representing a poker hand which implements PartialOrd.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    let mut hands: Vec<Hand> = hands
        .into_iter()
        .filter_map(|hand| {
            let cards = parse_hand(hand)?;
            Some(Hand::new(cards, hand))
        })
        .collect();
    hands.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut res = vec![];
    res.push(hands.pop().unwrap());
    while !hands.is_empty() && res.last() == hands.last() {
        res.push(hands.pop().unwrap());
    }
    Some(res.into_iter().map(|x| x.display).collect())
}

fn parse_hand<'a>(hand: &'a str) -> Option<[Card; 5]> {
    let mut res = vec![];
    for card in hand.split(" ") {
        if card.len() != 2 {
            return None;
        }
        if let Some(rank) = card.chars().nth(0)?.to_digit(10) {
            res.push(Card::new(rank as u8, card.chars().nth(1)?));
            continue;
        }

        let rank = match card.chars().nth(0)? {
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => return None,
        };
        res.push(Card::new(rank as u8, card.chars().nth(1)?));
    }
    res.retain(|x| "HSCD".contains(x.suit));
    if res.len() != 5 {
        return None;
    }
    let mut arr = [Card::default(); 5];
    res.into_iter().enumerate().for_each(|x| arr[x.0] = x.1);
    Some(arr)
}

#[test]
fn test_parse_hand() {
    let hand = "4S 5S 7H 8D JC";
    let res = parse_hand(hand);
    assert!(res.is_some());
    println!("{:?}", res);
}
