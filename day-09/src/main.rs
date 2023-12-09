use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek};

use itertools::Itertools;

fn extrapolate(values: &[i64]) -> i64 {
    if values.is_empty() {
        return 0;
    }

    let mut last_row = values.to_owned();
    let mut diagonal = Vec::new();
    while last_row.iter().any(|&x| x != 0) {
        diagonal.push(*last_row.last().unwrap());
        last_row = last_row.iter().copied()
            .tuple_windows::<(_, _)>()
            .map(|(a, b)| b - a)
            .collect_vec();
    }

    diagonal.into_iter().sum()
}

fn run<R: Read>(input: R, backwards: bool) -> Result<i64, aoc::Error> {
    let lines = BufReader::new(input).lines();
    let mut total = 0;
    for line in lines {
        let line = line?;
        let mut nums: Vec<i64> = aoc::parse_spaced(&line)?;
        if backwards {
            nums.reverse();
        }
        total += extrapolate(&nums);
    }

    Ok(total)
}

fn main() -> Result<(), aoc::Error> {
    let path = aoc::find_input_path("day-09");
    let mut f = File::open(path)?;

    // Answer: 1995001648
    let answer = run(&f, false)?;
    println!("Part 1: {answer}");
    f.rewind()?;
    // Answer: 988
    let answer = run(&f, true)?;
    println!("Part 2: {answer}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use indoc::indoc;

    use super::*;

    const EXAMPLE: &str = indoc! {"
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
    "};

    #[test]
    fn part1_example() {
        let total = run(Cursor::new(EXAMPLE), false).unwrap();
        assert_eq!(total, 114);
    }

    #[test]
    fn part2_example() {
        let total = run(Cursor::new(EXAMPLE), true).unwrap();
        assert_eq!(total, 2);
    }
}
