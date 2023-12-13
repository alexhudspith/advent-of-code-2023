use std::cmp::min;
use std::fs::File;
use std::io::{BufReader, Read, Seek};

use aoc::grid::{Axis, read_grid};

type Grid = aoc::grid::Grid<u8>;

fn solve(grid: &Grid, axis: Axis, require_smudge: bool) -> Option<usize> {
    for i in 0..grid.len(axis) - 1 {
        if is_reflection(grid, axis, i, require_smudge) {
            return Some(i + 1);
        }
    }

    None
}

fn run<R: Read>(input: R, require_smudge: bool) -> Result<usize, aoc::Error> {
    let mut reader = BufReader::new(input);
    let mut total = 0;
    loop {
        let grid: Grid = read_grid(&mut reader, None)?;
        if grid.is_empty() {
            break;
        }

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

fn is_reflection(grid: &Grid, axis: Axis, reflect_ix: usize, require_smudge: bool) -> bool {
    let offset_top = min(grid.len(axis) - reflect_ix, reflect_ix + 2);
    if offset_top <= 1 {
        return false;
    }

    let mut has_diff = false;
    for offset in 1..offset_top {
        let line1 = grid.get(axis, reflect_ix + offset);
        let line2 = grid.get(axis, reflect_ix + 1 - offset);
        for (&a, &b) in itertools::zip_eq(line1, line2) {
            if a != b {
                if !require_smudge || has_diff {
                    return false;
                }
                has_diff = true;
            }
        }
    }

    require_smudge == has_diff
}

fn main() -> Result<(), aoc::Error> {
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
