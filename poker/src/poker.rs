use std::cmp::Ordering;
use std::collections::HashMap;
use HandType::*;

#[derive(Debug, Clone, Copy)]
pub struct Card {
    pub rank: u8,
    pub suit: char,
}

impl Card {
    pub fn new(rank: u8, suit: char) -> Self {
        Self { rank, suit }
    }
}

impl Default for Card {
    fn default() -> Self {
        Self { rank: 2, suit: 'H' }
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

fn find_type(cards: &[Card]) -> (HandType, Vec<u8>) {
    let mut groups = HashMap::new();
    cards
        .iter()
        .for_each(|x| *groups.entry(x.rank).or_insert(0u8) += 1);
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

    if cards.windows(2).all(|x| x[0].rank - x[1].rank == 1)
        || cards[0].rank == 14
            && cards[1].rank == 5
            && cards[2].rank == 4
            && cards[3].rank == 3
            && cards[4].rank == 2
    {
        if cards[0].rank == 14 && cards[1].rank == 5 {
            extra.remove(0);
            extra.push(1);
        }
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
