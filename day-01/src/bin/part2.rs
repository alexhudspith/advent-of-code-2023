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

const DIGITS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn rev(s: &str) -> String {
    s.chars().rev().collect()
}

struct NumberSearch {
    regex: Regex,
    digit_map: HashMap<String, u64>,
}

impl NumberSearch {
    fn find(&self, s: &str) -> Option<u64> {
        let m = self.regex.find(s)?;
        self.digit_map.get(m.as_str()).copied()
    }
}

impl NumberSearch {
    pub fn new(digits: &[String]) -> Self {
        let mut digit_map: HashMap<String, u64> = digits.iter()
            .enumerate()
            .map(|(i, s)| (s.to_string(), i as u64 + 1))
            .collect();

        for i in 0..10 {
            digit_map.insert(format!("{i}"), i);
        }

        let pattern = format!(r#"\d|{}"#, digits.join("|"));
        Self {
            regex: Regex::new(&pattern).unwrap(),
            digit_map,
        }
    }
}

pub struct BidiNumberSearch {
    forward: NumberSearch,
    reverse: NumberSearch,
}

impl BidiNumberSearch {
    pub fn new() -> Self {
        let digits: Vec<_> = DIGITS.iter().map(|&s| s.to_string()).collect();
        let digits_rev: Vec<_> = DIGITS.iter().map(|&s| rev(s)).collect();
        Self {
            forward: NumberSearch::new(&digits),
            reverse: NumberSearch::new(&digits_rev),
        }
    }

    pub fn find(&self, line: &str) -> Option<(u64, u64)> {
        let first = self.forward.find(line)?;
        let last = self.reverse.find(&rev(line))?;

        Some((first, last))
    }
}

impl Default for BidiNumberSearch {
    fn default() -> Self {
        Self::new()
    }
}

fn run<R: Read>(input: R) -> io::Result<u64> {
    let lines = BufReader::new(input).lines();
    let searcher = BidiNumberSearch::new();
    let mut total = 0;
    for line in lines {
        let line = line?;
        let (first, last) = searcher.find(&line).ok_or_else(io_invalid)?;
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
