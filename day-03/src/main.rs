use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs::File;
use std::str;

use day_03::{number_spans, maybe_gear, is_symbol, Schematic, ColSpan, find_in_frame, read_schematic};

pub fn run<F>(schematic: &Schematic, mut score: F) -> Result<u64, aoc::error::Error>
    where F: FnMut(&Schematic, u64, ColSpan) -> u64
{
    let mut total = 0;
    for (r, row) in schematic.iter_rows().enumerate() {
        for col_span in number_spans(r, row.iter().enumerate()) {
            let digits = str::from_utf8(&row[col_span.start..col_span.end])?;
            total += score(schematic, digits.parse()?, col_span);
        }
    }

    Ok(total)
}


pub fn part1_fn() -> impl FnMut(&Schematic, u64, ColSpan) -> u64 {
    |schematic, number, col_span| {
        find_in_frame(schematic, is_symbol, col_span)
            .map(|_| number)
            .unwrap_or(0)
    }
}


pub fn part2_fn() -> impl FnMut(&Schematic, u64, ColSpan) -> u64 {
    let mut numbers_by_gear_pos = HashMap::new();

    move |schematic, number, col_span| {
        let gear_pos = find_in_frame(schematic, maybe_gear, col_span);
        if let Some((gr, gc)) = gear_pos {
            match numbers_by_gear_pos.entry((gr, gc)) {
                Entry::Occupied(entry) => { return entry.remove() * number; }
                Entry::Vacant(entry) => { entry.insert(number); }
            };
        }

        0
    }
}

fn main() -> Result<(), aoc::error::Error> {
    let path = aoc::find_input_path("day-03");
    let f = File::open(path)?;
    let s = read_schematic(f)?;

    // Answer: 536202
    let total = run(&s, part1_fn())?;
    println!("Part 1: {}", total);
    // Answer: 78272573
    let total = run(&s, part2_fn())?;
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
        let schematic = read_schematic(r).unwrap();
        let v = run(&schematic, part1_fn()).unwrap();
        assert_eq!(v, 4361);
    }

    #[test]
    fn part2_example() {
        let r = Cursor::new(EXAMPLE);
        let schematic = read_schematic(r).unwrap();
        let v = run(&schematic, part2_fn()).unwrap();
        assert_eq!(v, 467835);
    }
}
