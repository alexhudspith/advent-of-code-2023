use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs::File;
use std::path::{Path, PathBuf};

use day_03::{number_spans, maybe_gear, is_symbol, Schematic};

pub fn data_dir() -> PathBuf {
    Path::new(file!()).ancestors().nth(2).unwrap().join("data")
}

pub fn run<F>(schematic: &Schematic, mut f: F) -> anyhow::Result<u64>
    where F: FnMut(u64, usize, usize, usize) -> u64
{
    let mut total = 0;
    for (row, line) in schematic.iter_rows().enumerate() {
        for (start, end) in number_spans(line.iter().enumerate()) {
            let digits = std::str::from_utf8(&line[start..end])?;
            total += f(digits.parse()?, row, start, end);
        }
    }

    Ok(total)
}

pub fn part1_fn<'s>(schematic: &'s Schematic) -> impl FnMut(u64, usize, usize, usize) -> u64 + 's {
    |number, row, start_col, end_col| {
        schematic.find_in_frame(is_symbol, row, start_col, end_col)
            .map(|_| number)
            .unwrap_or(0)
    }
}

pub fn part2_fn<'s>(schematic: &'s Schematic) -> impl FnMut(u64, usize, usize, usize) -> u64 + 's {
    let mut numbers_by_gear_pos = HashMap::new();

    move |number, row, start_col, end_col| {
        let gear_pos = schematic.find_in_frame(maybe_gear, row, start_col, end_col);
        if let Some((gr, gc)) = gear_pos {
            match numbers_by_gear_pos.entry((gr, gc)) {
                Entry::Occupied(entry) => { return entry.remove() * number; }
                Entry::Vacant(entry) => { entry.insert(number); }
            };
        }

        0
    }
}

fn main() -> Result<(), anyhow::Error> {
    let f = File::open(data_dir().join("input.txt"))?;
    let s = Schematic::read(f)?;
    let total = run(&s, part1_fn(&s))?;
    println!("Part 1: {}", total);
    let total = run(&s, part2_fn(&s))?;
    println!("Part 2: {}", total);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
    "};

    #[test]
    fn part1_example() {
        let r = Cursor::new(EXAMPLE);
        let schematic = Schematic::read(r).unwrap();
        let v = run(&schematic, part1_fn(&schematic)).unwrap();
        assert_eq!(v, 4361);
    }

    #[test]
    fn part2_example() {
        let r = Cursor::new(EXAMPLE);
        let schematic = Schematic::read(r).unwrap();
        let v = run(&schematic, part2_fn(&schematic)).unwrap();
        assert_eq!(v, 467835);
    }
}
