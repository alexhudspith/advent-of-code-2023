// https://adventofcode.com/2023/day/1
// Part 2
// Answer: 54265

use std::collections::HashMap;
use regex::{Match, Regex};
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

fn digit(digit_words: &HashMap<&str, UInt>, m: &Match) -> Option<UInt> {
    digit_words.get(m.as_str()).copied()
}

pub fn find_digits_fn() -> impl FnMut(&str) -> Option<(UInt, UInt)> {
    let digit_words: HashMap<_, _> = DIGITS.into_iter().collect();
    let digits_alt = digit_words.keys().copied().collect::<Vec<_>>().join("|");
    let pattern_first = format!(r#"({digits_alt})"#);
    let pattern_last = format!(r#"(?:.*)({digits_alt})"#);
    let regex_first = Regex::new(&pattern_first).unwrap();
    let regex_last = Regex::new(&pattern_last).unwrap();

    move |line: &str| {
        let first_match = regex_first.captures(line)?.get(1)?;
        let first = digit(&digit_words, &first_match)?;
        let last = regex_last.captures_at(line, first_match.end())
            .and_then(|captures| captures.get(1))
            .and_then(|last_match| digit(&digit_words, &last_match))
            .unwrap_or(first);
        Some((first, last))
    }
}
