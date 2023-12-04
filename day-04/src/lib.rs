use std::collections::HashSet;
use std::error::Error;
use std::io;
use std::io::{BufRead, BufReader, Read};
use std::num::ParseIntError;
use std::str::FromStr;

use itertools::Itertools;

pub type Cards = Vec<Card>;

fn invalid_data<E: Error + Send + Sync + 'static>(e: E) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, e)
}

fn parse_numbers(line: &str) -> Result<HashSet<u8>, ParseIntError> {
    line.split_ascii_whitespace().map(|n| n.parse()).try_collect()
}

fn parse_cards(lines: impl Iterator<Item=String>) -> Result<Vec<Card>, ParseIntError> {
    lines.map(|line| line.parse()).try_collect()
}

pub fn read_cards<R: Read>(input: R) -> Result<Cards, io::Error> {
    BufReader::new(input).lines()
        .process_results(|lines|
            parse_cards(lines).map_err(invalid_data)
        )?
}

pub struct Card {
    pub win_count: u32,
    pub copies: u32,
}

impl FromStr for Card {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse_error = || "".parse::<i32>().unwrap_err();

        let (_, numbers) = s.split_once(':').ok_or_else(parse_error)?;
        let (winning, have) = numbers.split_once('|').ok_or_else(parse_error)?;
        let winning = parse_numbers(winning)?;
        let have = parse_numbers(have)?;
        let win_count: u32 = have.intersection(&winning)
            .count()
            .try_into()
            .expect("Win count too large");

        Ok(Card { win_count, copies: 1 })
    }
}
