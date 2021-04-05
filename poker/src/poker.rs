use std::collections::HashMap;
use std::str::FromStr;
use std::{cmp::Ordering, iter::FromIterator};
use HandType::*;

// diamonds (♦), clubs (♣), hearts (♥) and spades (♠)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Diamonds,
    Clubs,
    Hearts,
    Spades,
}

impl Suit {
    pub fn new(val: char) -> Self {
        match val {
            'D' => Suit::Diamonds,
            'C' => Suit::Clubs,
            'H' => Suit::Hearts,
            'S' => Suit::Spades,
            _ => panic!("Not standard suit"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Card {
    pub rank: u8,
    pub suit: Suit,
}

impl Card {
    pub fn new(rank: u8, suit: char) -> Self {
        let suit = Suit::new(suit);
        Self { rank, suit }
    }
}

impl Default for Card {
    fn default() -> Self {
        Self::new(2, 'H')
    }
}

impl FromStr for Card {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 3 {
            return Err(format!("invalid input: {}", s));
        }
        if s.len() == 3 {
            let rank = s.chars().take(2);
            let suit = s.chars().last().unwrap();
            if String::from_iter(rank) == "10" {
                return Ok(Card::new(10, suit));
            }
        }
        let mut chars = s.chars();
        let rank = chars.nth(0).unwrap();
        let suit = chars.last().unwrap();
        if let Some(rank) = rank.to_digit(10) {
            return Ok(Card::new(rank as u8, suit));
        }
        let rank: u8 = match rank {
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("invalid input"),
        };
        return Ok(Card::new(rank, suit));
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank
    }
}

impl Eq for Card {}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.rank.partial_cmp(&other.rank)
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank.cmp(&other.rank)
    }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    Three,
    Straight,
    Flush,
    FullHouse,
    Four,
    StraightFlush,
}

pub struct Hand<'a> {
    pub cards: [Card; 5],
    pub hand_type: HandType,
    pub extra: Vec<u8>,
    pub display: &'a str,
}

impl<'a> Hand<'a> {
    pub fn new(cards: [Card; 5], display: &'a str) -> Self {
        let mut cards = cards;
        cards.sort_by(|a, b| a.cmp(b));
        let (hand_type, extra) = find_type(&cards);
        Self {
            cards,
            hand_type,
            extra,
            display,
        }
    }

    pub fn from_str(value: &'a str) -> Option<Self> {
        if let Some(cards) = parse_hand(value) {
            return Some(Hand::new(cards, value));
        } else {
            return None;
        }
    }
}

impl PartialEq for Hand<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type as u8 == other.hand_type as u8 && self.extra == other.extra
    }
}

impl PartialOrd for Hand<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => self.extra.iter().cmp(other.extra.iter()),
            order => order,
        })
    }
}

fn parse_hand<'a>(hand: &'a str) -> Option<[Card; 5]> {
    let elements = hand.split(" ").collect::<Vec<_>>();
    if elements.len() != 5 || elements.iter().any(|e| e.len() > 3) {
        return None;
    }

    let cards = elements.iter().filter_map(|e| {
        if let Ok(card) = Card::from_str(e) {
            Some(card)
        } else {
            None
        }
    });
    let mut cards_arr = [Card::default(); 5];
    cards
        .enumerate()
        .for_each(|(idx, card)| cards_arr[idx] = card);
    Some(cards_arr)
}

fn find_type(cards: &[Card]) -> (HandType, Vec<u8>) {
    let groups = cards.iter().fold(HashMap::new(), |groups, card| {
        let mut groups = groups;
        *groups.entry(card.rank).or_insert(0u8) += 1;
        groups
    });
    let mut v: Vec<(u8, u8)> = groups.into_iter().collect();
    v.sort_by(|b, a| a.1.cmp(&b.1));
    let mut acc: Vec<(HandType, Vec<u8>)> = vec![];

    if v[0].1 == 4 {
        acc.push((Four, vec![v[0].0, v[1].0]));
    }

    if v[0].1 == 3 && v[1].1 == 2 {
        acc.push((FullHouse, vec![v[0].0, v[1].0]));
    }

    if v[0].1 == 3 && v[1].1 != 2 {
        let mut ranks = vec![v[1].0, v[2].0];
        ranks.sort_by(|b, a| a.cmp(b));
        acc.push((Three, vec![v[0].0, ranks[0], ranks[1]]));
    }

    if v[0].1 == 2 && v.len() == 3 {
        let mut ranks = vec![v[1].0, v[0].0];
        ranks.sort_by(|b, a| a.cmp(b));
        acc.push((TwoPair, vec![ranks[0], ranks[1], v[2].0]));
    }

    if v[0].1 == 2 && v.len() == 4 {
        let mut ranks = vec![v[1].0, v[2].0, v[3].0];
        ranks.sort_by(|b, a| a.cmp(b));
        acc.push((OnePair, vec![v[0].0, ranks[0], ranks[1], ranks[2]]));
    }

    let mut extra: Vec<u8> = cards.iter().map(|x| x.rank).collect();
    extra.sort_by(|b, a| a.cmp(b));

    if v.len() == 5 {
        acc.push((HighCard, extra.clone()));
    }

    let mut straight = false;
    let mut flush = false;

    if cards.iter().all(|x| cards[0].suit == x.suit) {
        flush = true;
    }

    println!("cards: {:?}", cards);
    if cards.windows(2).all(|x| {
        if let Some(x1) = x.first() {
            if let Some(x2) = x.last() {
                return x1.rank.checked_sub(x2.rank) == Some(1)
                    || x2.rank.checked_sub(x1.rank) == Some(1);
            }
        }
        return false;
    }) {
        straight = true;
    }

    let mut ranks: Vec<u8> = cards.iter().map(|card| card.rank).collect();
    ranks.sort_by(|a, b| b.cmp(a));
    if ranks[0] == 14 && ranks[1] == 5 && ranks[2] == 4 && ranks[3] == 3 && ranks[4] == 2 {
        extra.remove(0);
        extra.push(1);
        straight = true;
    }

    let hand_type = if straight && flush {
        Some(StraightFlush)
    } else if straight {
        Some(Straight)
    } else if flush {
        Some(Flush)
    } else {
        None
    };

    if let Some(raw_type) = hand_type {
        acc.push((raw_type, extra));
    }

    acc.sort_by(|b, a| a.0.cmp(&b.0));
    acc[0].clone()
}

#[test]
fn test_parse_hand() {
    let hand = "4S 5S 7H 8D JC";
    let res = parse_hand(hand);
    assert!(res.is_some());
    println!("{:?}", res);
}

#[test]
fn test_find_type() {
    let hand = "10D JH QS KD AC";
    let cards = parse_hand(hand).unwrap();
    let res = find_type(&cards);
    println!("{:?}", res);
    assert_eq!(res.0, HandType::Straight);

    let hand = "4S 5C 4C 5D 4H";
    let cards = parse_hand(hand).unwrap();
    let res = find_type(&cards);
    println!("{:?}", res);
    assert_eq!(res.0, HandType::FullHouse);

    let hand = "3H 6H 7H 8H 5H";
    let cards = parse_hand(hand).unwrap();
    let res = find_type(&cards);
    println!("{:?}", res);
    assert_eq!(res.0, HandType::Flush);

    let hand = "4D AH 3S 2D 5C";
    let cards = parse_hand(hand).unwrap();
    let res = find_type(&cards);
    println!("{:?}", res);
    assert_eq!(res.0, HandType::Straight);
}
