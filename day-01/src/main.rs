use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek};
use aoc::aoc_err;

mod part1;
mod part2;

pub type UInt = u64;

fn run<R, F>(input: R, mut find_digits: F) -> Result<UInt, aoc::Error>
    where R: Read, F: FnMut(&str) -> Option<(UInt, UInt)>
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

fn main() -> Result<(), aoc::Error> {
    let path = aoc::find_input_path("day-01");
    let mut f = File::open(path)?;

    let total = run(&f, part1::find_digits_fn())?;
    println!("Part 1: {total}");
    f.rewind()?;
    let total = run(&f, part2::find_digits_fn())?;
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
        let actual = run(input, part1::find_digits_fn()).unwrap();
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
        let actual = run(input, part2::find_digits_fn()).unwrap();
        assert_eq!(actual, 281);
    }
}
