use std::fmt::Debug;
use std::num::ParseIntError;

use indoc::indoc;
use itertools::{Itertools, zip_eq};
use aoc::aoc_err;
use aoc::parse::parse_spaced_vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Race {
    time: u64,
    distance: u64
}

fn quadratic_roots(b: f64, c: f64) -> Option<(f64, f64)> {
    let disc_sqrt = (b * b - 4.0 * c).sqrt();
    if disc_sqrt.is_nan() {
        return None;
    }

    let r1 = (-b - disc_sqrt) / 2.0;
    let r2 = (-b + disc_sqrt) / 2.0;
    Some((r1, r2))
}

fn win_count(race: Race) -> u64 {
    let t = race.time as f64;
    let k = race.distance as f64;
    // Solve for integer h, hold time:
    // distance = h(t - h) > k => h^2 - ht + k < 0
    if let Some((h1, h2)) = quadratic_roots(-t, k) {
        (h2.ceil() as u64 - 1) - (h1.floor() as u64 + 1) + 1
    } else {
        // No real roots => distance was impossible => no wins
        0
    }
}

fn squashed(line: &str) -> Result<u64, ParseIntError> {
    line.chars().filter(|&c| !c.is_whitespace()).collect::<String>().parse()
}

fn parse_races(s: &str, squash_space: bool) -> Result<Vec<Race>, aoc::Error> {
    let lines = s.split_terminator('\n')
        .filter(|line| !line.is_empty())
        .collect_vec();

    let [times, distances] = lines.as_slice() else { return Err(aoc_err("Incorrect line count")) };
    let times = times.strip_prefix("Time:").ok_or_else(|| aoc_err("No Time row"))?;
    let distances = distances.strip_prefix("Distance:").ok_or_else(|| aoc_err("No Distances row"))?;

    let result = if squash_space {
        vec![Race { time: squashed(times)?, distance: squashed(distances)? }]
    } else {
        zip_eq(parse_spaced_vec(times)?, parse_spaced_vec(distances)?)
            .map(|(time, distance)| Race { time, distance })
            .collect_vec()
    };

    Ok(result)
}

// Answer: 211904
fn part1(s: &str) -> Result<u64, aoc::Error> {
    let races = parse_races(s, false)?;
    Ok(races.into_iter().map(win_count).product())
}

// Answer: 43364472
fn part2(s: &str) -> Result<u64, aoc::Error> {
    let races = parse_races(s, true)?;
    assert_eq!(races.len(), 1);
    Ok(win_count(races[0]))
}

fn main() -> Result<(), aoc::Error> {
    let s = indoc!{"
        Time:        56     71     79     99
        Distance:   334   1135   1350   2430
    "};

    let answer = part1(s)?;
    println!("Part 1: {answer}");
    let answer = part2(s)?;
    println!("Part 1: {answer}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = indoc! {"
        Time:      7  15   30
        Distance:  9  40  200
    "};

    #[test]
    fn part1_example() {
        let total = part1(EXAMPLE).unwrap();
        assert_eq!(total, 288);
    }

    #[test]
    fn part2_example() {
        let total = part2(EXAMPLE).unwrap();
        assert_eq!(total, 71503);
    }
}
