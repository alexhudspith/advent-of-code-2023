// https://adventofcode.com/2023/day/1
// Part 2
// Answer: 54265

#![allow(dead_code)]

use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::{BufReader, Cursor};
use regex::Regex;
use day_01::{data_dir, io_invalid};

const DIGITS: [(&str, u64); 18] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
];

const DIGITS_REV: [(&str, u64); 18] = [
    ("eno", 1),
    ("owt", 2),
    ("eerht", 3),
    ("ruof", 4),
    ("evif", 5),
    ("xis", 6),
    ("neves", 7),
    ("thgie", 8),
    ("enin", 9),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
];

fn rev(s: &str) -> String {
    s.chars().rev().collect()
}

struct NumberSearch<'d> {
    digit_words: HashMap<&'d str, u64>,
    regex: Regex,
}

impl<'d> NumberSearch<'d> {
    pub fn new(digit_words: HashMap<&'d str, u64>) -> Self {
        let pattern = digit_words.keys().copied().collect::<Vec<_>>().join("|");
        Self {
            digit_words,
            regex: Regex::new(&pattern).unwrap(),
        }
    }

    fn find(&self, s: &str) -> Option<u64> {
        let m = self.regex.find(s)?;
        self.digit_words.get(m.as_str()).copied()
    }
}

fn run<R: Read>(input: R) -> io::Result<u64> {
    let lines = BufReader::new(input).lines();
    let forward = NumberSearch::new(DIGITS.into_iter().collect());
    let reverse = NumberSearch::new(DIGITS_REV.into_iter().collect());

    let mut total = 0;
    for line in lines {
        let line = line?;
        let first = forward.find(&line).ok_or_else(io_invalid)?;
        let last = reverse.find(&rev(&line)).expect("Already found first");
        total += first * 10 + last;
    }

    Ok(total)
}

fn run_str(input: &str) -> io::Result<u64> {
    run(Cursor::new(input))
}

fn main() -> io::Result<()> {
    let path = data_dir().join("input.txt");
    let f = File::open(path)?;
    let total = run(f)?;
    println!("{total}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let actual = run_str(input).unwrap();
        assert_eq!(actual, 281);
    }
}
