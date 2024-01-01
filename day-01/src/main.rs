use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek};
use regex::{Match, Regex};
use aoc::error::aoc_err;

fn run<R, F>(input: R, mut find_digits: F) -> Result<usize, aoc::error::Error>
    where
        R: Read,
        F: FnMut(&str) -> Option<(usize, usize)>
{
    let lines = BufReader::new(input).lines();
    let mut total = 0;
    for line in lines {
        let line = line?;
        let (first, last) = find_digits(&line).ok_or_else(|| aoc_err("No digits in line"))?;
        total += first * 10 + last;
    }

    Ok(total)
}

fn find_digits(line: &str) -> Option<(usize, usize)> {
    let first = line.chars()
        .flat_map(|c: char| c.to_digit(10))
        .next()?;
    let last = line.chars().rev()
        .flat_map(|c: char| c.to_digit(10))
        .next()
        .expect("Already found first");

    Some((first as usize, last as usize))
}

const DIGITS: [(&str, usize); 18] = [
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

fn digit(digit_words: &HashMap<&str, usize>, m: &Match) -> Option<usize> {
    digit_words.get(m.as_str()).copied()
}

fn find_digit_words_fn() -> impl FnMut(&str) -> Option<(usize, usize)> {
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

// Answer: 54450
fn part1<R: Read>(input: R) -> Result<usize, aoc::error::Error> {
    run(input, find_digits)
}

// Answer: 54265
fn part2<R: Read>(input: R) -> Result<usize, aoc::error::Error> {
    run(input, find_digit_words_fn())
}

fn main() -> Result<(), aoc::error::Error> {
    let path = aoc::find_input_path("day-01");
    let mut f = File::open(path)?;

    let total = part1(&f)?;
    println!("Part 1: {total}");
    f.rewind()?;
    let total = part2(&f)?;
    println!("Part 2: {total}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use indoc::indoc;

    #[test]
    fn part1_example() {
        let input = Cursor::new(indoc!("
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        "));
        let actual = part1(input).unwrap();
        assert_eq!(actual, 142);
    }

    #[test]
    fn part2_example() {
        let input = Cursor::new(indoc!("
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        "));
        let actual = part2(input).unwrap();
        assert_eq!(actual, 281);
    }
}
