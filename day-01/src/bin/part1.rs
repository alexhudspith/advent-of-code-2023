// https://adventofcode.com/2023/day/1
// Part 2
// Answer: 54450

#![allow(dead_code)]

use std::fs::File;
use std::io;
use std::io::{BufReader, Cursor};
use std::io::prelude::*;

use day_01::*;

pub fn find_digits(line: &str) -> Option<(&str, &str)> {
    let pos = line.find(|c: char| c.is_ascii_digit())?;
    let first = &line[pos..pos + 1];

    let pos = line.rfind(|c: char| c.is_ascii_digit())?;
    let last = &line[pos..pos + 1];

    Some((first, last))
}

fn run<R: Read>(input: R) -> io::Result<u64> {
    let lines = BufReader::new(input).lines();
    let mut total = 0;
    for line in lines {
        let line = line?;
        let (first, last) = find_digits(&line).ok_or_else(io_invalid)?;
        let number = format!("{first}{last}");
        let result: u64 = number.parse().map_err(io_invalid_with)?;
        total += result;
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
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let actual = run_str(input).unwrap();
        assert_eq!(actual, 142);
    }
}
