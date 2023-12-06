use std::fmt::Debug;
use std::str::FromStr;

use anyhow::{anyhow, bail};
use indoc::indoc;
use itertools::{Itertools, zip_eq};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Race {
    time: u64,
    distance: u64
}

fn quadratic_roots(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let disc = (b.powi(2) - 4.0 * a * c).sqrt();
    if disc.is_nan() {
        return None;
    }

    let r1 = (-b + disc) / (2.0 * a);
    let r2 = (-b - disc) / (2.0 * a);
    Some((r1, r2))
}

fn win_count(race: Race) -> u64 {
    let t = race.time as f64;
    let k = race.distance as f64;
    if let Some((a, b)) = quadratic_roots(1.0, -t, k) {
        (a.ceil() as u64 - 1) - (b.floor() as u64 + 1) + 1
    } else {
        // No real roots => distance was impossible => no wins
        0
    }
}

fn numbers<T: FromStr>(line: &str) -> Result<Vec<T>, T::Err> where T::Err: Debug {
    line.split_ascii_whitespace().map(|n| n.parse::<T>()).try_collect()
}

fn squashed_number<T: FromStr>(line: &str) -> Result<T, T::Err> where T::Err: Debug {
    line.chars()
        .filter(|&c| !c.is_whitespace())
        .collect::<String>()
        .parse()
}

fn parse_races(s: &str, keep_space: bool) -> anyhow::Result<Vec<Race>> {
    let lines = s.split_terminator('\n')
        .filter(|line| !line.is_empty())
        .collect_vec();

    let [times, distances] = lines.as_slice() else { bail!("Incorrect line count") };
    let times = times.strip_prefix("Time:").ok_or_else(|| anyhow!("No Time row"))?;
    let distances = distances.strip_prefix("Distance:").ok_or_else(|| anyhow!("No Distances row"))?;

    let result = if keep_space {
        zip_eq(numbers(times)?, numbers(distances)?)
            .map(|(time, distance)| Race { time, distance })
            .collect_vec()
    } else {
        vec![Race { time: squashed_number(times)?, distance: squashed_number(distances)? }]
    };

    Ok(result)
}

// Answer: 211904
fn part1(s: &str) -> anyhow::Result<u64> {
    let races = parse_races(s, true)?;
    Ok(races.into_iter().map(win_count).product())
}

// Answer: 43364472
fn part2(s: &str) -> anyhow::Result<u64> {
    let races = parse_races(s, false)?;
    assert_eq!(races.len(), 1);
    Ok(win_count(races[0]))
}

fn main() -> anyhow::Result<()> {
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
