// https://adventofcode.com/2023/day/1
// Part 2
// Answer: 54265

use std::collections::HashMap;
use regex::Regex;
use crate::UInt;

const DIGITS: [(&str, UInt); 18] = [
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

const DIGITS_REV: [(&str, UInt); 18] = [
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
    digit_words: HashMap<&'d str, UInt>,
    regex: Regex,
}

impl<'d> NumberSearch<'d> {
    pub fn new(digit_words: HashMap<&'d str, UInt>) -> Self {
        let pattern = digit_words.keys().copied().collect::<Vec<_>>().join("|");
        Self {
            digit_words,
            regex: Regex::new(&pattern).unwrap(),
        }
    }

    fn find(&self, s: &str) -> Option<UInt> {
        let m = self.regex.find(s)?;
        self.digit_words.get(m.as_str()).copied()
    }
}

pub(crate) fn find_digits_fn() -> impl FnMut(&str) -> Option<(UInt, UInt)> {
    let forward = NumberSearch::new(DIGITS.into_iter().collect());
    let reverse = NumberSearch::new(DIGITS_REV.into_iter().collect());

    move |line: &str| {
        let first = forward.find(line)?;
        let last = reverse.find(&rev(line)).expect("Already found first");
        Some((first, last))
    }
}
