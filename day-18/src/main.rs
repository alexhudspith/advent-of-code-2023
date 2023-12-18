use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek};

use itertools::Itertools;

use aoc::{aoc_err, CollectArray};
use aoc::grid::Way;

type Position = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Instruction {
    way: Way,
    count: usize,
}

fn from_hex(s: &str) -> Result<usize, String> {
    usize::from_str_radix(s, 16).map_err(|_| s.into())
}

fn parse_line_part1(line: &str) -> Result<Instruction, aoc::Error> {
    let [way_str, count_str, _] = line.split_ascii_whitespace()
        .try_collect_array()
        .map_err(|_| aoc_err(format!("Bad line {line}")))?;

    let way = way_str.parse()?;
    let count = count_str.parse()?;

    Ok(Instruction { way, count })
}

fn parse_line_part2(line: &str) -> Result<Instruction, aoc::Error> {
    let [_, _, colour_str] = line.split_ascii_whitespace()
        .try_collect_array()
        .map_err(|_| aoc_err(format!("Bad line {line}")))?;

    let colour_str = colour_str
        .strip_prefix("(#")
        .and_then(|s| s.strip_suffix(')'))
        .ok_or_else(|| aoc_err(format!("Bad colour: {colour_str}")))?;

    let count = from_hex(&colour_str[..5])?;
    let way = match colour_str.chars().last().unwrap() {
        '0' => Way::Right,
        '1' => Way::Down,
        '2' => Way::Left,
        '3' => Way::Up,
        _ => return Err(format!("Bad line {line}").into()),
    };

    Ok(Instruction { way, count })
}


fn parse_instructions<R, F>(input: R, mut parse_line: F) -> Result<Vec<Instruction>, aoc::Error>
    where
        R: Read,
        F: FnMut(&str) -> Result<Instruction, aoc::Error>
{
    let lines = BufReader::new(input).lines();
    let mut table = vec![];
    for line in lines {
        let line = line?;
        table.push(parse_line(&line)?);
    }
    Ok(table)
}

fn signed(pos: Position) -> (isize, isize) {
    (pos.0 as isize, pos.1 as isize)
}

fn run<R, F>(input: R, parse_line: F) -> Result<usize, aoc::Error>
    where
        R: Read,
        F: FnMut(&str) -> Result<Instruction, aoc::Error>
{
    let table = parse_instructions(input, parse_line)?;
    let origin = (1 << 24, 1 << 24);
    let perim: usize = perimeter(&table);
    let interior2x: isize = iter_position_loop(origin, table).tuple_windows()
        .map(|(pos1, pos2)| (signed(pos1), signed(pos2)))
        .map(|(pos1, pos2)| (pos1.0 * pos2.1) - (pos2.0 * pos1.1))
        .sum();

    let area = (interior2x.unsigned_abs() + perim) / 2 + 1;
    Ok(area)
}

// Answer: 61865
fn part1<R: Read>(input: R) -> Result<usize, aoc::Error> {
    run(input, parse_line_part1)
}

fn part2<R: Read>(input: R) -> Result<usize, aoc::Error> {
    run(input, parse_line_part2)
}

fn iter_instr<'i, I>(start: Position, table: I) -> impl Iterator<Item=Position> + 'i
    where
        I: IntoIterator<Item=Instruction> + 'i
{
    table.into_iter().scan(start, |pos, instr: I::Item| {
        let result = *pos;
        *pos = instr.way.steps(result, instr.count);
        Some(result)
    })
}

fn iter_position_loop<'i, I>(start: Position, table: I) -> impl Iterator<Item=Position> + 'i
    where
        I: IntoIterator<Item=Instruction> + 'i
{
    let mut it = iter_instr(start, table).peekable();
    let first = it.peek().copied();
    it.chain(first)
}

fn perimeter(table: &[Instruction]) -> usize {
    let p: usize = table.iter().map(|instr| instr.count).sum();
    p
}

fn main() -> Result<(), aoc::Error> {
    let path = aoc::find_input_path("day-18");
    let mut f = File::open(path)?;

    // Answer: 61865
    let answer = part1(&f)?;
    println!("Part 1: {answer}");
    f.rewind()?;
    // Answer: 40343619199142
    let answer = part2(&f)?;
    println!("Part 2: {answer}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use indoc::indoc;

    const EXAMPLE: &str = indoc!{r"
        R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)
    "};

    const WHY: &str = indoc!{r"
        R 3 (#000000)
        U 3 (#000000)
        R 3 (#000000)
        U 4 (#000000)
        L 3 (#000000)
        D 2 (#000000)
        L 3 (#000000)
        U 2 (#000000)
        L 3 (#000000)
        D 4 (#000000)
        R 3 (#000000)
        D 3 (#000000)
    "};

    const WHY_NOT: &str = indoc!{r"
        R 3 (#000000)
        D 3 (#000000)
        R 3 (#000000)
        D 4 (#000000)
        L 3 (#000000)
        U 2 (#000000)
        L 3 (#000000)
        D 2 (#000000)
        L 3 (#000000)
        U 4 (#000000)
        R 3 (#000000)
        U 3 (#000000)
    "};

    const SQUARE1: &str = indoc!{r"
        R 3 (#000000)
        D 3 (#000000)
        L 3 (#000000)
        U 3 (#000000)
   "};

    #[test]
    fn square() {
        let answer = part1(Cursor::new(SQUARE1)).unwrap();
        assert_eq!(answer, 16);
    }

    #[test]
    fn why() {
        let answer = part1(Cursor::new(WHY)).unwrap();
        assert_eq!(answer, 58);
    }

    #[test]
    fn why_not() {
        let answer = part1(Cursor::new(WHY_NOT)).unwrap();
        assert_eq!(answer, 58);
    }

    #[test]
    fn part1_example() {
        let answer = part1(Cursor::new(EXAMPLE)).unwrap();
        assert_eq!(answer, 62);
    }

    #[test]
    fn part2_example() {
        let answer = part2(Cursor::new(EXAMPLE)).unwrap();
        assert_eq!(answer, 952408144115);
    }
}
