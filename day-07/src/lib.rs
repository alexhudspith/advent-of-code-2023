#![allow(clippy::redundant_field_names)]

use std::io::{BufRead, BufReader, Read};
use std::str;
use std::str::FromStr;
use itertools::Itertools;
use aoc::error::aoc_err;
use aoc::CollectArray;

const CARDS: [char; 14] = ['*', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];
const JOKER_STR: &str = "*";
const JOKER_ORD: usize = 0;
const HAND_SIZE: usize = 5;

type HandArray<T> = [T; HAND_SIZE];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card { ord: usize }

impl TryFrom<char> for Card {
    type Error = char;

    fn try_from(chr: char) -> Result<Self, Self::Error> {
        CARDS.iter()
            .position(|&b| b == chr)
            .map(|ord| Self { ord })
            .ok_or(chr)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    // Ordered by increasing hand strength
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

pub fn counts(cards: &HandArray<Card>) -> [u8; CARDS.len()] {
    let mut counts = [0; CARDS.len()];
    for card in cards {
        counts[card.ord] += 1;
    }
    counts
}

pub fn chosen_joker_hand(cards: &HandArray<Card>) -> HandArray<Card> {
    let counts = counts(cards);
    if counts[JOKER_ORD] == 0 {
        return *cards;
    }

    // position_max returns the last if multiple elements are equal 😀
    let mode_ord_no_joker = counts.iter()
        .skip(1)
        .position_max()
        .unwrap() + 1;

    let joker = Card { ord: mode_ord_no_joker };
    cards.iter()
        .map(|&c| if c.ord == JOKER_ORD { joker } else { c })
        .collect_array()
}

fn hand_type(cards: &HandArray<Card>) -> HandType {
    let new_cards = chosen_joker_hand(cards);
    let counts = counts(&new_cards);
    let distinct = counts.iter().copied().filter(|&c| c != 0).count();
    let max_run = counts.iter().copied().max().unwrap();

    match (distinct, max_run) {
        (1, _) => HandType::FiveKind,
        (2, 4) => HandType::FourKind,
        (2, 3) => HandType::FullHouse,
        (3, 3) => HandType::ThreeKind,
        (3, 2) => HandType::TwoPair,
        (4, _) => HandType::OnePair,
        (5, _) => HandType::HighCard,
        _ => unreachable!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hand {
    // Fields ordered for hand strength
    ty: HandType,
    cards: HandArray<Card>,
}

impl Hand {
    pub fn new(cards: HandArray<Card>) -> Self {
        Self {
            ty: hand_type(&cards),
            cards: cards
        }
    }
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != HAND_SIZE {
            return Err(s.to_owned());
        }

        let cards = s.chars()
            .map(Card::try_from)
            .process_results(|cards| cards.collect_array())
            .map_err(|_| s.to_owned())?;

        Ok(Self::new(cards))
    }
}

fn parse_line(line: &str, use_jokers: bool) -> Result<(Hand, u32), aoc::error::Error> {
    let split = line.split_ascii_whitespace().collect_vec();
    let &[hand, bid] = split.as_slice() else {
        return Err(aoc_err("Bad line"));
    };

    let hand: Hand = if use_jokers {
        hand.replace('J', JOKER_STR).parse()?
    } else {
        hand.parse()?
    };

    let bid: u32 = bid.parse()?;
    Ok((hand, bid))
}

pub fn read_hand_bids<R: Read>(input: R, use_jokers: bool) -> Result<Vec<(Hand, u32)>, aoc::error::Error> {
    let lines = BufReader::new(input).lines();
    lines
        .process_results(|lines| {
            let hand_bids: Vec<(Hand, u32)> = lines.into_iter()
                .map(|line: String| parse_line(&line, use_jokers))
                .try_collect()?;
            Ok(hand_bids)
        })?
}
