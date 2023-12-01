use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read, Seek};
use std::path::{Path, PathBuf};

mod part1;
mod part2;

pub type UInt = u64;

pub fn data_dir() -> PathBuf {
    Path::new(file!()).ancestors().nth(2).unwrap().join("data")
}

fn run<R, F>(input: R, mut find_digits: F) -> io::Result<UInt>
    where R: Read, F: FnMut(&str) -> Option<(UInt, UInt)>
{
    let lines = BufReader::new(input).lines();
    let mut total = 0;
    for line in lines {
        let line = line?;
        let (first, last) = find_digits(&line)
            .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidData))?;
        total += first * 10 + last;
    }

    Ok(total)
}

fn main() -> io::Result<()> {
    let path = data_dir().join("input.txt");
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

    #[test]
    fn part1() {
        let input = Cursor::new("1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet");
        let actual = run(input, part1::find_digits_fn()).unwrap();
        assert_eq!(actual, 142);
    }

    #[test]
    fn part2() {
        let input = Cursor::new("two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen");
        let actual = run(input, part2::find_digits_fn()).unwrap();
        assert_eq!(actual, 281);
    }
}