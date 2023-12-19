use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

use itertools::Itertools;

use aoc::aoc_err;
use aoc::parse::{parse_lines, parse_spaced};

pub type Cards = Vec<Card>;

pub fn read_cards<R: Read>(input: R) -> Result<Cards, aoc::Error> {
    BufReader::new(input).lines().process_results(|lines| parse_lines(lines))?
}

pub struct Card {
    pub win_count: u32,
    pub copies: u32,
}

impl FromStr for Card {
    type Err = aoc::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, numbers) = s.split_once(':').ok_or(aoc_err("No colon"))?;
        let (winning, have) = numbers.split_once('|').ok_or(aoc_err("No pipe"))?;
        let winning: HashSet<u32> = parse_spaced(winning)?;
        let have: HashSet<u32> = parse_spaced(have)?;
        let win_count: u32 = have.intersection(&winning)
            .count()
            .try_into()
            .expect("Win count too large");

        Ok(Card { win_count, copies: 1 })
    }
}
