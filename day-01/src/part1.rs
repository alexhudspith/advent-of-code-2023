// https://adventofcode.com/2023/day/1
// Part 1
// Answer: 54450

use crate::UInt;

fn find_digits(line: &str) -> Option<(UInt, UInt)> {
    let first = line.chars()
        .flat_map(|c: char| c.to_digit(10))
        .next()?;
    let last = line.chars().rev()
        .flat_map(|c: char| c.to_digit(10))
        .next()
        .expect("Already found first");

    Some((first as UInt, last as UInt))
}

pub(crate) fn find_digits_fn() -> impl FnMut(&str) -> Option<(UInt, UInt)> {
    find_digits
}
