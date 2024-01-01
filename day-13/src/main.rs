use std::fs::File;
use std::io::{BufReader, Read, Seek};
use itertools::Itertools;
use aoc::CumulativeExt;

use aoc::grid::{Axis, read_grid_ascii};

type Grid = aoc::grid::Grid<u8>;

fn solve(grid: &Grid, axis: Axis, require_smudge: bool) -> Option<usize> {
    let diffs_required = if require_smudge { 1 } else { 0 };

    (0..grid.len(axis) - 1).filter_map(|i| {
        is_reflection(grid, axis, i, diffs_required).then_some(i + 1)
    }).next()
}

fn run<R: Read>(input: R, require_smudge: bool) -> Result<usize, aoc::error::Error> {
    let mut reader = BufReader::new(input);
    let mut total = 0;
    loop {
        let grid = match read_grid_ascii(&mut reader, None) {
            Ok(grid) => grid,
            Err(aoc::error::Error::EndOfFile) => break,
            Err(e) => return Err(e),
        };

        // eprintln!("{}", &grid);
        total += solve(&grid, Axis::Row, require_smudge)
            .map_or_else(
                || solve(&grid, Axis::Column, require_smudge),
                |r| Some(100 * r),
            )
            .unwrap_or(0);
    }

    Ok(total)
}

fn is_reflection(grid: &Grid, axis: Axis, reflect_ix: usize, diffs_required: usize) -> bool {
    let reverse = (0..=reflect_ix).rev().map(|i| grid.get(axis, i).copied());
    let forward = (reflect_ix + 1..grid.len(axis)).map(|i| grid.get(axis, i).copied());
    let diffs: usize = reverse.zip(forward)
        .map(|(line1, line2)| diff_count(line1, line2, diffs_required + 1))
        .cumulative_sum()
        .take_while_inclusive(|&diffs| diffs <= diffs_required)
        .last()
        .expect("Expect at least 2 rows or columns");

    diffs == diffs_required
}

fn diff_count<I: Iterator<Item=u8>>(line1: I, line2: I, max_diffs: usize) -> usize {
    itertools::zip_eq(line1, line2)
        .filter(|&(cell1, cell2)| cell1 != cell2)
        .take(max_diffs)
        .count()
}

fn main() -> Result<(), aoc::error::Error> {
    let path = aoc::find_input_path("day-13");
    let mut f = File::open(path)?;

    // Answer: 27664
    let answer = run(&f, false)?;
    println!("Part 1: {answer}");
    f.rewind()?;
    // Answer: 33991
    let answer = run(&f, true)?;
    println!("Part 2: {answer}");
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use indoc::indoc;

    const EXAMPLE1: &str = indoc! {"
        #.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.

        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#
    "};

    #[test]
    fn part1_example() {
        let r = Cursor::new(EXAMPLE1);
        let answer = run(r, false).unwrap();
        assert_eq!(answer, 405);
    }

    #[test]
    fn part2_example() {
        let r = Cursor::new(EXAMPLE1);
        let answer = run(r, true).unwrap();
        assert_eq!(answer, 400);
    }
}
